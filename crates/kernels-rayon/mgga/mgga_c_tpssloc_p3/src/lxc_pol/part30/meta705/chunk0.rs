//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2308/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2308(t5932: f64, t6743: f64, t28653: f64, t82822: f64, t1014: f64, t1058: f64, t1060: f64, t11046: f64, t14608: f64, t1625: f64, t17959: f64, t18093: f64, t1945: f64, t23478: f64, t23601: f64, t23602: f64, t23633: f64, t25492: f64, t25516: f64, t25554: f64, t25558: f64, t25712: f64, t28596: f64, t28601: f64, t28641: f64, t3186: f64, t4673: f64, t6687: f64, t82717: f64, t89175: f64, t89224: f64) -> f64 {
    let t100204 = t6743 * t5932;
    let t100215 = t82822 * t28653;
    let t100225 = t89175 + 2.0_f64 * t3186 * t28641 * t4673 - 0.16449340668482264365e-1_f64 * t23601 * t23602 * t1014 * t1625 * t25492 + 0.54831135561607547883e-2_f64 * t23633 * t100204 * t25554 - 0.16449340668482264365e-1_f64 * t6687 * t25712 * t23478 * t25516 + 4.0_f64 * t3186 * t28601 * t4673 + 0.18277045187202515961e-2_f64 * t100215 - t89224 + t11046 * t28596 * t18093 - 0.18277045187202515961e-2_f64 * t82717 - 2.0_f64 * t14608 * t25558 + t1058 * t1945 * t17959 * t1060;
    t100225
}
