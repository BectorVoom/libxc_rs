//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 562/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk562(t1019: f64, t1023: f64, t1022: f64, t404: f64, t394: f64, t392: f64, t395: f64, t2834: f64, t2509: f64, t275: f64, t400: f64, t1039: f64, t673: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2857 = t1019 * t1023;
    let t2860 = t1022 * t404;
    let t2861 = 1.0_f64 / t2860;
    let t2862 = t394 * t2861;
    let t2868 = 1.0_f64 / t395 / t392;
    let t2872 = 4.0_f64 / 9.0_f64 * t2834;
    let t2880 = 0.39862222222222222223e0_f64 * t2834;
    let t2885 = 1.0_f64/f64::sqrt(t392);
    let t2891 = t275 * t2509 * t400;
    let t2892 = 0.13692777777777777778e0_f64 * t2891;
    let t2893 = t673 * t1039;
    (t2857, t2861, t2862, t2868, t2872, t2880, t2885, t2891, t2892, t2893)
}
