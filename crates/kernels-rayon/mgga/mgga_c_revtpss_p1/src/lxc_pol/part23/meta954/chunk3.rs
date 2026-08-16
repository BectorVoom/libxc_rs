//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3176/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3176(t1261: f64, t24240: f64, t247: f64, t3634: f64, t21192: f64, t5381: f64, t1469: f64, t17736: f64, t17737: f64, t17763: f64, t20806: f64, t20838: f64, t21017: f64, t21306: f64, t24726: f64, t3367: f64, t3626: f64, t3647: f64, t4181: f64, t5245: f64, t5354: f64, t6573: f64, t6673: f64, t6683: f64, t70623: f64, t71513: f64) -> f64 {
    let t83392 = t1261 * t247 * t3634 * t24240;
    let t83394 = t5381 * t21192;
    let t83414 = -0.85748036236139473944e-3_f64 * t17763 * t6683 - 0.85748036236139473944e-3_f64 * t3647 * t24726 - 0.85748036236139473944e-3_f64 * t70623 - 0.57165357490759649296e-3_f64 * t83392 - 0.57165357490759649296e-3_f64 * t83394 + 0.7145669686344956162e-3_f64 * t17763 * t6673 - 0.12862205435420921092e-2_f64 * t21306 * t20838 - 0.21722835846488666732e-1_f64 * t71513 * t5354 + 0.34299214494455789577e-2_f64 * t21017 * t20806 - 0.17149607247227894789e-2_f64 * t17736 * t3626 * t17737 * t1469 * t5245 - 0.17149607247227894789e-2_f64 * t17736 * t3626 * t6573 * t3367 * t4181;
    t83414
}
