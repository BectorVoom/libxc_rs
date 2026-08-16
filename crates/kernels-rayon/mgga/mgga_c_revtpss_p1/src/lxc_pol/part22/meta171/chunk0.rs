//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1129/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1129(t4029: f64, t512: f64, t1320: f64, t1331: f64, t1340: f64, t2516: f64) -> (f64, f64, f64, f64, f64) {
    let t4030 = t512 * t4029;
    let t4031 = 2.0_f64 * t4030;
    let t4032 = t1320 * t1331;
    let t4033 = 8.0_f64 * t4032;
    let t4035 = 0.5848223622634646207e0_f64 * t1340 * t2516;
    (t4030, t4031, t4032, t4033, t4035)
}
