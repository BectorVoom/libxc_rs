//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1996/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1996(t95822: f64, t98815: f64, t95537: f64, t25310: f64, t28360: f64, t25365: f64, t26485: f64, t99466: f64, t28377: f64, t689: f64, t25431: f64, t102930: f64, t102934: f64, t102937: f64, t14979: f64, t7403: f64, t95538: f64, t95542: f64, t95543: f64, t95548: f64) -> (f64, f64) {
    let t102939 = 0.28912093960683998208e-1_f64 * t95822 * t98815;
    let t102941 = 0.51405703062096148812e-1_f64 * t95537 * t98815;
    let t102943 = 0.14456046980341999104e-1_f64 * t25310 * t28360;
    let t102945 = 0.25702851531048074406e-1_f64 * t25365 * t28360;
    let t102947 = 0.28912093960683998208e-1_f64 * t99466 * t26485;
    let t102951 = t28377 * t689;
    let t102953 = 0.14456046980341999104e-1_f64 * t25431 * t102951;
    let t102954 = -0.51405703062096148812e-1_f64 * t95538 - t102930 + t102934 - t102937 + t102939 - t102941 + t102943 - t102945 - t102947 - t95542 - 0.12851425765524037203e-1_f64 * t95543 - 0.65854491829355115987e0_f64 * t7403 * t14979 - t95548 - t102953;
    (t102951, t102954)
}
