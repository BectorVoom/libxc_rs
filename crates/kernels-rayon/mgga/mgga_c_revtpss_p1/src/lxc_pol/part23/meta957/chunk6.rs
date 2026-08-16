//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3207/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3207(t84156: f64, t84174: f64, t1234: f64, t24680: f64, t1222: f64, t140: f64, t24826: f64, t1235: f64, t1238: f64, t17283: f64, t21085: f64, t21236: f64, t24636: f64, t3667: f64, t371: f64, t372: f64, t482: f64, t5323: f64, t5348: f64, t5373: f64, t59419: f64, t59426: f64, t6647: f64, t71513: f64, t72064: f64, t72071: f64) -> (f64, f64) {
    let t84175 = t84156 + t84174;
    let t84185 = t1234 * t24680;
    let t84195 = t1222 * t140 * t24826;
    let t84197 = -0.10162730220579493208e-2_f64 * t59419 - 0.85748036236139473944e-3_f64 * t72064 - 0.21437009059034868486e-3_f64 * t3667 * t24636 - 0.21437009059034868486e-3_f64 * t1235 * t371 * t372 * t482 * t84175 + 0.34299214494455789577e-2_f64 * t17283 * t6647 + 0.34299214494455789577e-2_f64 * t5323 * t21085 + 0.53100265402527852012e-1_f64 * t84185 * t1238 - 2.0_f64 / 27.0_f64 * t5373 * t21236 + 0.42874018118069736972e-3_f64 * t72071 - 0.21722835846488666732e-1_f64 * t71513 * t5348 - 0.95275595817932748827e-4_f64 * t59426 - 7.0_f64 / 1944.0_f64 * t84195;
    (t84175, t84197)
}
