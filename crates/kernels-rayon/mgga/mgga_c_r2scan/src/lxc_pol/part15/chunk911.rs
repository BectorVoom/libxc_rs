//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 911/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk911(t1020: f64, t1312: f64, t1022: f64, t1024: f64, t1026: f64, t1028: f64, t1310: f64, t2414: f64, t2418: f64, t2422: f64, t839: f64, t8438: f64) -> (f64, f64) {
    let t8440 = t1312 * t1020;
    let t8454 = 0.734774460522e2_f64 * t1022 * t1312 - 0.11494261417236e3_f64 * t1024 * t1312 + 0.6202613620464e2_f64 * t1026 * t1312 - 0.1088826475632e2_f64 * t1028 * t1312 - 0.64e0_f64 * t8438 - 0.9214113627294e1_f64 * t8440 - 0.18428227254588e2_f64 * t2414 * t839 - 0.9214113627294e1_f64 * t1022 * t1310 + 0.734774460522e2_f64 * t2418 * t839 + 0.367387230261e2_f64 * t1024 * t1310 - 0.7662840944824e2_f64 * t2422 * t839 - 0.3831420472412e2_f64 * t1026 * t1310;
    (t8440, t8454)
}
