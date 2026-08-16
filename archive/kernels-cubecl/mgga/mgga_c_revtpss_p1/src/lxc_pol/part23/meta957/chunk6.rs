//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3207/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3207<F: Float>(t84156: F, t84174: F, t1234: F, t24680: F, t1222: F, t140: F, t24826: F, t1235: F, t1238: F, t17283: F, t21085: F, t21236: F, t24636: F, t3667: F, t371: F, t372: F, t482: F, t5323: F, t5348: F, t5373: F, t59419: F, t59426: F, t6647: F, t71513: F, t72064: F, t72071: F) -> (F, F) {
    let t84175 = t84156 + t84174;
    let t84185 = t1234 * t24680;
    let t84195 = t1222 * t140 * t24826;
    let t84197 = -F::cast_from(0.10162730220579493208e-2_f64) * t59419 - F::cast_from(0.85748036236139473944e-3_f64) * t72064 - F::cast_from(0.21437009059034868486e-3_f64) * t3667 * t24636 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t371 * t372 * t482 * t84175 + F::cast_from(0.34299214494455789577e-2_f64) * t17283 * t6647 + F::cast_from(0.34299214494455789577e-2_f64) * t5323 * t21085 + F::cast_from(0.53100265402527852012e-1_f64) * t84185 * t1238 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5373 * t21236 + F::cast_from(0.42874018118069736972e-3_f64) * t72071 - F::cast_from(0.21722835846488666732e-1_f64) * t71513 * t5348 - F::cast_from(0.95275595817932748827e-4_f64) * t59426 - F::cast_from(7.0_f64) / F::cast_from(1944.0_f64) * t84195;
    (t84175, t84197)
}
