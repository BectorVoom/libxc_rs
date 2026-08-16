//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1032/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1032(t12531: f64, t12546: f64, t1188: f64, t1178: f64, t3519: f64, t439: f64, t3522: f64, t447: f64, t12487: f64, t1161: f64, t1180: f64, t1189: f64, t12429: f64, t12431: f64, t12465: f64, t12470: f64, t12473: f64, t12476: f64, t12481: f64, t12486: f64, t12488: f64, t12491: f64, t12494: f64, t12497: f64, t12501: f64, t12504: f64, t12508: f64, t12511: f64, t12514: f64, t3452: f64, t3454: f64, t3477: f64, t3491: f64, t3496: f64, t3498: f64, t3516: f64, t3521: f64, t3524: f64) -> (f64, f64, f64, f64) {
    let t12547 = t12531 + t12546;
    let t12548 = t12547 * t1188;
    let t12552 = 1.0_f64 / t3519 / t1178;
    let t12553 = t439 * t12552;
    let t12555 = 1.0_f64 / t3522 / t447;
    let t12556 = t12487 * t12555;
    let t12559 = -0.19298375398431042081e3_f64 * t12429 * t12431 + 1.0_f64 * t1161 * t12465 + 0.2069040516770936012e4_f64 * t12470 * t12473 + 0.17544670867903938621e1_f64 * t12476 * t1189 + 0.17544670867903938621e1_f64 * t3491 * t3516 + 0.51947577317044391276e2_f64 * t12481 * t3524 - 0.10389515463408878255e3_f64 * t12486 * t12488 - 0.35089341735807877242e1_f64 * t12491 * t3498 + 0.35089341735807877242e1_f64 * t3521 * t12494 - 0.35089341735807877242e1_f64 * t3496 * t12497 + 0.51947577317044391277e2_f64 * t3521 * t12501 - 6.0_f64 * t3452 * t12504 + 0.96491876992155210402e2_f64 * t3477 * t12508 - 6.0_f64 * t12511 * t3454 + 6.0_f64 * t3477 * t12514 + 0.5848223622634646207e0_f64 * t1180 * t12548 + 0.10254018858216406658e4_f64 * t12553 * t12556;
    (t12547, t12552, t12555, t12559)
}
