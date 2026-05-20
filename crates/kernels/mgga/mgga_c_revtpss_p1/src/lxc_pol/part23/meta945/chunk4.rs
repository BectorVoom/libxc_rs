//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3108/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3108<F: Float>(t1168: F, t1187: F, t12470: F, t12481: F, t12486: F, t12491: F, t16965: F, t17097: F, t17154: F, t1756: F, t1757: F, t20382: F, t20615: F, t20659: F, t20662: F, t20671: F, t20672: F, t24363: F, t24414: F, t24423: F, t24436: F, t3452: F, t3496: F, t3521: F, t5180: F, t5181: F, t58237: F, t58259: F, t6502: F, t6519: F, t6534: F, t69504: F, t81566: F) -> F {
    let t81835 = -F::cast_from(0.31168546390226634766e3_f64) * t58259 * t20672 - t81566 + F::cast_from(0.10526802520742363173e2_f64) * t17097 * t20659 - F::cast_from(0.70178683471615754484e1_f64) * t17154 * t20662 - F::cast_from(0.14035736694323150897e2_f64) * t12486 * t24436 * t1187 + F::cast_from(0.10526802520742363173e2_f64) * t3521 * t6519 * t5180 - F::cast_from(0.35089341735807877242e1_f64) * t12491 * t24423 - F::cast_from(0.35089341735807877242e1_f64) * t3496 * t5181 * t6534 - F::cast_from(0.35089341735807877242e1_f64) * t3496 * t1757 * t20382 + F::cast_from(0.51947577317044391277e2_f64) * t12481 * t24414 + F::cast_from(0.51947577317044391277e2_f64) * t3521 * t69504 * t1756 + F::cast_from(0.51947577317044391277e2_f64) * t3521 * t20671 * t5180 + F::new(18.0) * t58237 * t20615 + F::cast_from(0.62071215503128080361e4_f64) * t12470 * t6502 * t16965 * t1168 - F::new(2.0) * t3452 * t24363 * t1168;
    t81835
}
