//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 238/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk238(t446: f64, t500: f64, t385: f64, t422: f64, t388: f64, t421: f64, t155: f64, t389: f64, t1002: f64, t1004: f64, t1011: f64, t1014: f64, t1017: f64, t1019: f64, t1021: f64, t1022: f64, t436: f64, t948: f64, t975: f64, t982: f64, t998: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1023 = t500 * t446;
    let t1027 = 8.0_f64 * t385 * t422;
    let t1028 = t388 * t421;
    let t1029 = t155 * t1028;
    let t1030 = 2.0_f64 * t1029;
    let t1031 = t385 * t389;
    let t1032 = 8.0_f64 * t1031;
    let t1033 = t948 - t975 + t982 + 0.93273e-1_f64 * t436 * t998 - 0.31091e-1_f64 * t1002 * t1004 + t1011 + t1014 - t1017 + t1019 + t1021 + 0.186546e0_f64 * t1022 * t1023 - t1027 + t1030 - t1032;
    (t1023, t1027, t1028, t1029, t1030, t1031, t1032, t1033)
}
