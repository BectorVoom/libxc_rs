//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1785/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1785<F: Float>(t3628: F, t5825: F, t6573: F, t1235: F, t17709: F, t1791: F, t20851: F, t20956: F, t21063: F, t24636: F, t371: F, t372: F, t3720: F, t44844: F, t482: F, t5327: F, t6611: F, t6647: F, t70263: F, t70278: F, t70578: F, t83109: F, t84098: F, t84636: F, t89960: F) -> (F, F, F) {
    let t91012 = t3628 * t5825;
    let t91037 = t6573 * t6573;
    let t91060 = F::cast_from(0.28582678745379824648e-3_f64) * t70263 - F::cast_from(0.3811023832717309953e-3_f64) * t70278 + F::cast_from(0.77173232612525526552e-2_f64) * t17709 * t3720 * t20956 * t84636 + F::cast_from(0.51448821741683684368e-2_f64) * t44844 * t371 * t372 * t482 * t91037 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t371 * t372 * t482 * t89960 - F::cast_from(0.85748036236139473944e-3_f64) * t83109 * t1791 - F::cast_from(0.12862205435420921092e-2_f64) * t20851 * t6647 - F::cast_from(0.85748036236139473944e-3_f64) * t5327 * t24636 + F::cast_from(0.25724410870841842184e-2_f64) * t70578 * t6611 + F::cast_from(0.13719685797782315831e-1_f64) * t84098 * t1791 + F::cast_from(0.13719685797782315831e-1_f64) * t21063 * t6647;
    (t91012, t91037, t91060)
}
