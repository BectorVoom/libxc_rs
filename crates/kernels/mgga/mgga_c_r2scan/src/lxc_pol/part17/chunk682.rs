//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 682/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk682<F: Float>(t5251: F, t597: F, t5250: F, t1823: F, t732: F, t1818: F, t712: F, t1822: F, t234: F, t716: F, t224: F, t719: F) -> (F, F, F, F, F, F, F) {
    let t5252 = t597 * t5251;
    let t5253 = t5250 * t5252;
    let t5258 = t732 * t1823;
    let t5260 = t1818 * t712;
    let t5261 = t5260 * t1822;
    let t5263 = F::cast_from(0.30762056574649219973e4_f64) * t234 * t5261;
    let t5265 = t716 * t716;
    let t5266 = F::new(1.0) / t5265;
    let t5267 = t5266 * t224;
    let t5268 = t719 * t719;
    (t5252, t5253, t5258, t5263, t5266, t5267, t5268)
}
