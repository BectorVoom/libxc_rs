//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 515/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk515(t531: f64, t549: f64, t240: f64, t72: f64, t1386: f64, t2482: f64, t27: f64, t136: f64, t1389: f64, t1317: f64, t1333: f64, t1340: f64, t2516: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4010 = 1.0_f64 / t549 / t531;
    let t4011 = t240 * t4010;
    let t4012 = t4011 * t72;
    let t4018 = t2482 * t1386 * t27;
    let t4019 = t1389 * t136;
    let t4027 = 8.0_f64 * t1317 * t1333;
    let t4035 = 0.5848223622634646207e0_f64 * t1340 * t2516;
    (t4010, t4011, t4012, t4018, t4019, t4027, t4035)
}
