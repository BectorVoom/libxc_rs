//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1198/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1198(t24021: f64, t256: f64, t2494: f64, t2427: f64, t2472: f64, t23465: f64, t23468: f64, t23789: f64, t23817: f64, t24223: f64, t24225: f64, t24228: f64, t24230: f64, t24233: f64, t24299: f64, t24308: f64, t2476: f64, t2520: f64, t2530: f64, t2534: f64, t2537: f64, t2538: f64, t7504: f64, t7748: f64, t7799: f64, t7805: f64, t7813: f64, t7825: f64, t837: f64) -> (f64, f64) {
    let t24733 = t256 * t24021;
    let t24737 = t2494 * t2494;
    let t24743 = t2427 * t2472;
    let t24748 = t23465 - t23468 - 0.1403573615389248977e2_f64 * t7813 * t23789 * t837 - 0.35089340384731224426e1_f64 * t2530 * t23817 * t837 + 0.51947267698127589897e2_f64 * t2537 * t23817 * t2476 - t24223 - t24225 - t24228 - t24230 - t24233 - t24299 - t24308 - 0.12304676425209353917e5_f64 * t24733 * t23789 * t7504 + 0.11579285944033451271e4_f64 * t7799 * t24737 * t2520 + 0.35089340384731224426e1_f64 * t7805 * t2534 + 0.1038945353962551798e3_f64 * t24743 * t2538 + 24.0_f64 * t7825 * t7748;
    (t24737, t24748)
}
