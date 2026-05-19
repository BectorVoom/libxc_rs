//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1198/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1198<F: Float>(t24021: F, t256: F, t2494: F, t2427: F, t2472: F, t23465: F, t23468: F, t23789: F, t23817: F, t24223: F, t24225: F, t24228: F, t24230: F, t24233: F, t24299: F, t24308: F, t2476: F, t2520: F, t2530: F, t2534: F, t2537: F, t2538: F, t7504: F, t7748: F, t7799: F, t7805: F, t7813: F, t7825: F, t837: F) -> (F, F) {
    let t24733 = t256 * t24021;
    let t24737 = t2494 * t2494;
    let t24743 = t2427 * t2472;
    let t24748 = t23465 - t23468 - F::cast_from(0.1403573615389248977e2_f64) * t7813 * t23789 * t837 - F::cast_from(0.35089340384731224426e1_f64) * t2530 * t23817 * t837 + F::cast_from(0.51947267698127589897e2_f64) * t2537 * t23817 * t2476 - t24223 - t24225 - t24228 - t24230 - t24233 - t24299 - t24308 - F::cast_from(0.12304676425209353917e5_f64) * t24733 * t23789 * t7504 + F::cast_from(0.11579285944033451271e4_f64) * t7799 * t24737 * t2520 + F::cast_from(0.35089340384731224426e1_f64) * t7805 * t2534 + F::cast_from(0.1038945353962551798e3_f64) * t24743 * t2538 + F::new(24.0) * t7825 * t7748;
    (t24737, t24748)
}
