//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1785/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1785(t3628: f64, t5825: f64, t6573: f64, t1235: f64, t17709: f64, t1791: f64, t20851: f64, t20956: f64, t21063: f64, t24636: f64, t371: f64, t372: f64, t3720: f64, t44844: f64, t482: f64, t5327: f64, t6611: f64, t6647: f64, t70263: f64, t70278: f64, t70578: f64, t83109: f64, t84098: f64, t84636: f64, t89960: f64) -> (f64, f64, f64) {
    let t91012 = t3628 * t5825;
    let t91037 = t6573 * t6573;
    let t91060 = 0.28582678745379824648e-3_f64 * t70263 - 0.3811023832717309953e-3_f64 * t70278 + 0.77173232612525526552e-2_f64 * t17709 * t3720 * t20956 * t84636 + 0.51448821741683684368e-2_f64 * t44844 * t371 * t372 * t482 * t91037 - 0.21437009059034868486e-3_f64 * t1235 * t371 * t372 * t482 * t89960 - 0.85748036236139473944e-3_f64 * t83109 * t1791 - 0.12862205435420921092e-2_f64 * t20851 * t6647 - 0.85748036236139473944e-3_f64 * t5327 * t24636 + 0.25724410870841842184e-2_f64 * t70578 * t6611 + 0.13719685797782315831e-1_f64 * t84098 * t1791 + 0.13719685797782315831e-1_f64 * t21063 * t6647;
    (t91012, t91037, t91060)
}
