//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 655/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk655<F: Float>(t15: F, t3: F, t42: F, t148: F, t40: F, t5239: F, t1725: F, t58: F, t423: F, t170: F, t1727: F, t597: F, t1375: F, t1859: F, t1862: F, t1823: F, t732: F) -> (F, F, F, F, F, F, F, F) {
    let t5243 = 1.0 / t15 / t3 / t42 / 48.0;
    let t5244 = t148 * t5243;
    let t5245 = t3 * t40;
    let t5246 = t5244 * t5245;
    let t5248 = 0.42340699333333333333e-2 * t5239 * t5246;
    let t5249 = t1725 * t58;
    let t5250 = t5249 * t423;
    let t5251 = t170 * t1727;
    let t5252 = t597 * t5251;
    let t5253 = t5250 * t5252;
    let t5255 = t1859 * t1375;
    let t5256 = t5255 * t1862;
    let t5258 = t732 * t1823;
    (t5245, t5246, t5248, t5249, t5252, t5253, t5256, t5258)
}
