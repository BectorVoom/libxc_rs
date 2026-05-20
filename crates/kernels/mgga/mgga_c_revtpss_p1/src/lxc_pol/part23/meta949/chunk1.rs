//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3138/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3138<F: Float>(t21102: F, t5265: F, t20816: F, t5274: F, t1042: F, t1261: F, t17569: F, t17609: F, t20825: F, t20907: F, t20914: F, t21143: F, t24808: F, t3647: F, t5268: F, t5270: F, t5279: F, t5381: F, t6625: F, t69906: F, t80045: F, t80050: F) -> F {
    let t82441 = t21102 * t5265;
    let t82457 = t5274 * t20816;
    let t82467 = F::cast_from(0.14481890564325777821e-1_f64) * t82441 - F::cast_from(0.85748036236139473944e-3_f64) * t3647 * t24808 - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t1042 * t5268 * t80045 - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t1042 * t5268 * t80050 + F::cast_from(0.85748036236139473944e-3_f64) * t17569 * t20914 + F::cast_from(0.64311027177104605458e-3_f64) * t17609 * t6625 + F::cast_from(0.42874018118069736972e-3_f64) * t82457 - F::cast_from(0.85748036236139473944e-3_f64) * t5381 * t20907 - F::cast_from(0.85748036236139473944e-3_f64) * t21143 * t5270 - F::cast_from(0.7145669686344956162e-3_f64) * t17569 * t20825 + F::cast_from(0.42874018118069736972e-3_f64) * t69906 * t5279;
    t82467
}
