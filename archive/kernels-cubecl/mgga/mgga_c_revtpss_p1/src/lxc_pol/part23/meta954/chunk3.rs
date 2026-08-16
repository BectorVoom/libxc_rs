//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3176/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3176<F: Float>(t1261: F, t24240: F, t247: F, t3634: F, t21192: F, t5381: F, t1469: F, t17736: F, t17737: F, t17763: F, t20806: F, t20838: F, t21017: F, t21306: F, t24726: F, t3367: F, t3626: F, t3647: F, t4181: F, t5245: F, t5354: F, t6573: F, t6673: F, t6683: F, t70623: F, t71513: F) -> F {
    let t83392 = t1261 * t247 * t3634 * t24240;
    let t83394 = t5381 * t21192;
    let t83414 = -F::cast_from(0.85748036236139473944e-3_f64) * t17763 * t6683 - F::cast_from(0.85748036236139473944e-3_f64) * t3647 * t24726 - F::cast_from(0.85748036236139473944e-3_f64) * t70623 - F::cast_from(0.57165357490759649296e-3_f64) * t83392 - F::cast_from(0.57165357490759649296e-3_f64) * t83394 + F::cast_from(0.7145669686344956162e-3_f64) * t17763 * t6673 - F::cast_from(0.12862205435420921092e-2_f64) * t21306 * t20838 - F::cast_from(0.21722835846488666732e-1_f64) * t71513 * t5354 + F::cast_from(0.34299214494455789577e-2_f64) * t21017 * t20806 - F::cast_from(0.17149607247227894789e-2_f64) * t17736 * t3626 * t17737 * t1469 * t5245 - F::cast_from(0.17149607247227894789e-2_f64) * t17736 * t3626 * t6573 * t3367 * t4181;
    t83414
}
