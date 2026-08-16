//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 932/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk932(t1330: f64, t749: f64, t512: f64, t1320: f64, t1331: f64, t1340: f64, t2516: f64, t2496: f64, t177: f64, t762: f64, t2626: f64, t3827: f64, t3856: f64, t3859: f64, t3862: f64, t3865: f64, t3867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4029 = t1330 * t749;
    let t4030 = t512 * t4029;
    let t4031 = 2.0_f64 * t4030;
    let t4032 = t1320 * t1331;
    let t4033 = 8.0_f64 * t4032;
    let t4035 = 0.5848223622634646207e0_f64 * t1340 * t2516;
    let t4037 = 0.17315859105681463759e2_f64 * t1340 * t2496;
    let t4038 = t1330 * t177;
    let t4039 = t4038 * t762;
    let t4040 = 0.11696447245269292414e1_f64 * t4039;
    let t4042 = 0.11696447245269292414e1_f64 * t1340 * t2626;
    let t4043 = t3856 + t4031 - t4033 - t3867 - t4035 - t4037 - t4040 + t3859 + t3862 - t3865 - t3827 + t4042;
    (t4029, t4030, t4031, t4032, t4033, t4035, t4037, t4038, t4039, t4040, t4042, t4043)
}
