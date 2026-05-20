//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3161/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3161<F: Float>(t12553: F, t1756: F, t12428: F, t1737: F, t3495: F, t5155: F, t1160: F, t17020: F, t1170: F, t1187: F, t12430: F, t12431: F, t12470: F, t12481: F, t12486: F, t12487: F, t12491: F, t12547: F, t16982: F, t16988: F, t16992: F, t16997: F, t16998: F, t17026: F, t1757: F, t3453: F, t3472: F, t3477: F, t3496: F, t3497: F, t3498: F, t3515: F, t43977: F, t45174: F, t45177: F, t5143: F, t5146: F, t5181: F, t57907: F) -> F {
    let t58300 = t12553 * t1756;
    let t58304 = t1737 * t12428;
    let t58307 = t5155 * t3495;
    let t58310 = t17020 * t1160;
    let t58315 = F::new(18.0) * t3477 * t5143 * t3453 + F::cast_from(0.11579025239058625248e4_f64) * t12470 * t5146 * t12430 + F::cast_from(0.51947577317044391277e2_f64) * t12481 * t16992 + F::cast_from(0.30762056574649219973e4_f64) * t45174 * t16998 - F::cast_from(0.35089341735807877242e1_f64) * t3496 * t5181 * t3515 - F::cast_from(0.31168546390226634765e3_f64) * t12486 * t16988 * t3497 - F::cast_from(0.35089341735807877242e1_f64) * t12491 * t16982 - F::cast_from(0.11696447245269292414e1_f64) * t3496 * t1757 * t12547 - F::cast_from(0.12304822629859687989e5_f64) * t45177 * t16997 * t12487 + F::cast_from(0.30762056574649219974e4_f64) * t58300 * t43977 * t1187 + t57907 - F::cast_from(0.19298375398431042081e3_f64) * t58304 * t12431 - F::cast_from(0.35089341735807877242e1_f64) * t58307 * t3498 + F::new(3.0) * t58310 * t1170 + F::new(3.0) * t17026 * t3472;
    t58315
}
