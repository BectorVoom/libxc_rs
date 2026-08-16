//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3128/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3128(t12898: f64, t1786: f64, t17202: f64, t372: f64, t15936: f64, t5405: f64, t17708: f64, t45769: f64, t44546: f64, t5340: f64, t5342: f64, t13041: f64, t56730: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57615 = t1786 * t12898;
    let t57621 = t372 * t17202;
    let t57622 = t15936 * t5405;
    let t57631 = t45769 * t17708;
    let t57635 = t5340 * t44546 * t5342;
    let t57636 = 0.28582678745379824648e-3_f64 * t57635;
    let t57641 = t56730 * t13041;
    (t57615, t57621, t57622, t57631, t57636, t57641)
}
