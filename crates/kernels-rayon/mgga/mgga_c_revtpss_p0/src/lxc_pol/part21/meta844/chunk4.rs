//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3161/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3161(t12553: f64, t1756: f64, t12428: f64, t1737: f64, t3495: f64, t5155: f64, t1160: f64, t17020: f64, t1170: f64, t1187: f64, t12430: f64, t12431: f64, t12470: f64, t12481: f64, t12486: f64, t12487: f64, t12491: f64, t12547: f64, t16982: f64, t16988: f64, t16992: f64, t16997: f64, t16998: f64, t17026: f64, t1757: f64, t3453: f64, t3472: f64, t3477: f64, t3496: f64, t3497: f64, t3498: f64, t3515: f64, t43977: f64, t45174: f64, t45177: f64, t5143: f64, t5146: f64, t5181: f64, t57907: f64) -> f64 {
    let t58300 = t12553 * t1756;
    let t58304 = t1737 * t12428;
    let t58307 = t5155 * t3495;
    let t58310 = t17020 * t1160;
    let t58315 = 18.0_f64 * t3477 * t5143 * t3453 + 0.11579025239058625248e4_f64 * t12470 * t5146 * t12430 + 0.51947577317044391277e2_f64 * t12481 * t16992 + 0.30762056574649219973e4_f64 * t45174 * t16998 - 0.35089341735807877242e1_f64 * t3496 * t5181 * t3515 - 0.31168546390226634765e3_f64 * t12486 * t16988 * t3497 - 0.35089341735807877242e1_f64 * t12491 * t16982 - 0.11696447245269292414e1_f64 * t3496 * t1757 * t12547 - 0.12304822629859687989e5_f64 * t45177 * t16997 * t12487 + 0.30762056574649219974e4_f64 * t58300 * t43977 * t1187 + t57907 - 0.19298375398431042081e3_f64 * t58304 * t12431 - 0.35089341735807877242e1_f64 * t58307 * t3498 + 3.0_f64 * t58310 * t1170 + 3.0_f64 * t17026 * t3472;
    t58315
}
