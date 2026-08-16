//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1156/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1156(t2105: f64, t6470: f64, t1851: f64, t7961: f64, t112: f64, t29395: f64, t29376: f64, t532: f64, t2752: f64, t29105: f64, t225: f64, t29095: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100966 = t6470 * t2105;
    let t100972 = t1851 * t7961;
    let t100996 = t29395 * t112;
    let t101150 = t532 * t29376;
    let t101226 = t29105 * t2752;
    let t101355 = t29095 * t225;
    (t100966, t100972, t100996, t101150, t101226, t101355)
}
