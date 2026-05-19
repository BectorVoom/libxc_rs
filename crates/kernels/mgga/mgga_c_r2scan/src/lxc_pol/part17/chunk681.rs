//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 681/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk681<F: Float>(t159: F, t166: F, t15: F, t3: F, t42: F, t148: F, t40: F, t1725: F, t58: F, t423: F, t170: F, t1727: F) -> (F, F, F, F, F, F) {
    let t5239 = t159 * t166;
    let t5243 = F::new(1.0) / t15 / t3 / t42 / F::new(48.0);
    let t5244 = t148 * t5243;
    let t5245 = t3 * t40;
    let t5246 = t5244 * t5245;
    let t5248 = F::cast_from(0.42340699333333333333e-2_f64) * t5239 * t5246;
    let t5249 = t1725 * t58;
    let t5250 = t5249 * t423;
    let t5251 = t170 * t1727;
    (t5245, t5246, t5248, t5249, t5250, t5251)
}
