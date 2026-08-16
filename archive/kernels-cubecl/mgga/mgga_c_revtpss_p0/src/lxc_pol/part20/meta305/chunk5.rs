//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1199/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1199<F: Float>(t12487: F, t12555: F, t1161: F, t1180: F, t1189: F, t12429: F, t12431: F, t12465: F, t12470: F, t12473: F, t12476: F, t12481: F, t12486: F, t12488: F, t12491: F, t12494: F, t12497: F, t12501: F, t12504: F, t12508: F, t12511: F, t12514: F, t12548: F, t12553: F, t3452: F, t3454: F, t3477: F, t3491: F, t3496: F, t3498: F, t3516: F, t3521: F, t3524: F) -> (F, F) {
    let t12556 = t12487 * t12555;
    let t12559 = -F::cast_from(0.19298375398431042081e3_f64) * t12429 * t12431 + F::cast_from(1.0_f64) * t1161 * t12465 + F::cast_from(0.2069040516770936012e4_f64) * t12470 * t12473 + F::cast_from(0.17544670867903938621e1_f64) * t12476 * t1189 + F::cast_from(0.17544670867903938621e1_f64) * t3491 * t3516 + F::cast_from(0.51947577317044391276e2_f64) * t12481 * t3524 - F::cast_from(0.10389515463408878255e3_f64) * t12486 * t12488 - F::cast_from(0.35089341735807877242e1_f64) * t12491 * t3498 + F::cast_from(0.35089341735807877242e1_f64) * t3521 * t12494 - F::cast_from(0.35089341735807877242e1_f64) * t3496 * t12497 + F::cast_from(0.51947577317044391277e2_f64) * t3521 * t12501 - F::cast_from(6.0_f64) * t3452 * t12504 + F::cast_from(0.96491876992155210402e2_f64) * t3477 * t12508 - F::cast_from(6.0_f64) * t12511 * t3454 + F::cast_from(6.0_f64) * t3477 * t12514 + F::cast_from(0.5848223622634646207e0_f64) * t1180 * t12548 + F::cast_from(0.10254018858216406658e4_f64) * t12553 * t12556;
    (t12556, t12559)
}
