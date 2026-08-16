//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 955/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk955(t5: f64, t10309: f64, t13272: f64, t1497: f64, t21663: f64, t2247: f64, t22648: f64, t22656: f64, t22659: f64, t22742: f64, t4173: f64, t5816: f64, t5872: f64, t603: f64, t91: f64) -> f64 {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t22746 = piecewise3(t8, 0.0_f64, -120.0_f64 * t10309 * t22656 + 60.0_f64 * t13272 * t5816 - 12.0_f64 * t1497 * t21663 + 60.0_f64 * t2247 * t22659 + t22648 * t91 - 4.0_f64 * t22742 * t603 - 12.0_f64 * t4173 * t5872);
    t22746
}
