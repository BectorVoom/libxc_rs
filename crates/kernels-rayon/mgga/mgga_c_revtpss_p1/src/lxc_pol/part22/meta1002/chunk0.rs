//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3409/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3409(t19049: f64, t3030: f64, t19467: f64, t2989: f64, t981: f64, t19226: f64, t2970: f64, t11404: f64, t11409: f64, t11548: f64, t15252: f64, t15255: f64, t15413: f64, t19227: f64, t19272: f64, t19275: f64, t19276: f64, t19282: f64, t2943: f64, t2944: f64, t2962: f64, t2968: f64, t41667: f64, t41740: f64, t41742: f64, t52443: f64, t6157: f64, t6174: f64, t6177: f64, t63212: f64, t63214: f64, t63216: f64, t63218: f64, t63220: f64, t63222: f64, t63224: f64, t953: f64) -> (f64, f64, f64) {
    let t63940 = 0.5848223622634646207e0_f64 * t19049 * t3030;
    let t63943 = 0.35089341735807877242e1_f64 * t981 * t19467 * t2989;
    let t63971 = t19226 * t2970;
    let t63975 = 0.19964560303604640732e6_f64 * t41740 * t6157 * t41742 * t2944 - 0.23392894490538584828e1_f64 * t15413 * t15252 - 0.2077903092681775651e3_f64 * t52443 * t15255 + t63212 - t63214 + t63216 - t63218 - t63220 + t63222 + t63224 - 0.19298375398431042081e3_f64 * t11409 * t6177 * t2962 - 0.24828486201251232145e5_f64 * t41667 * t19282 * t2944 - 4.0_f64 * t11548 * t19272 - 4.0_f64 * t2943 * t19227 * t953 - 2.0_f64 * t2943 * t6174 * t2962 - 0.19298375398431042081e3_f64 * t11409 * t19275 * t2944 + 0.64327917994770140268e2_f64 * t11404 * t19276 + 0.64327917994770140268e2_f64 * t2968 * t63971 * t953;
    (t63940, t63943, t63975)
}
