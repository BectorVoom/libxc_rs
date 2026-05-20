//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3409/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3409<F: Float>(t19049: F, t3030: F, t19467: F, t2989: F, t981: F, t19226: F, t2970: F, t11404: F, t11409: F, t11548: F, t15252: F, t15255: F, t15413: F, t19227: F, t19272: F, t19275: F, t19276: F, t19282: F, t2943: F, t2944: F, t2962: F, t2968: F, t41667: F, t41740: F, t41742: F, t52443: F, t6157: F, t6174: F, t6177: F, t63212: F, t63214: F, t63216: F, t63218: F, t63220: F, t63222: F, t63224: F, t953: F) -> (F, F, F) {
    let t63940 = F::cast_from(0.5848223622634646207e0_f64) * t19049 * t3030;
    let t63943 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t19467 * t2989;
    let t63971 = t19226 * t2970;
    let t63975 = F::cast_from(0.19964560303604640732e6_f64) * t41740 * t6157 * t41742 * t2944 - F::cast_from(0.23392894490538584828e1_f64) * t15413 * t15252 - F::cast_from(0.2077903092681775651e3_f64) * t52443 * t15255 + t63212 - t63214 + t63216 - t63218 - t63220 + t63222 + t63224 - F::cast_from(0.19298375398431042081e3_f64) * t11409 * t6177 * t2962 - F::cast_from(0.24828486201251232145e5_f64) * t41667 * t19282 * t2944 - F::new(4.0) * t11548 * t19272 - F::new(4.0) * t2943 * t19227 * t953 - F::new(2.0) * t2943 * t6174 * t2962 - F::cast_from(0.19298375398431042081e3_f64) * t11409 * t19275 * t2944 + F::cast_from(0.64327917994770140268e2_f64) * t11404 * t19276 + F::cast_from(0.64327917994770140268e2_f64) * t2968 * t63971 * t953;
    (t63940, t63943, t63975)
}
