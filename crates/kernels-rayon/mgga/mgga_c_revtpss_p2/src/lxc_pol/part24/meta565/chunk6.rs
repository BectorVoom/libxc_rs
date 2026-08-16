//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1720/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1720(t1089: f64, t19526: f64, t19556: f64, t24083: f64, t24090: f64, t24104: f64, t24116: f64, t24138: f64, t24157: f64, t3204: f64, t43446: f64, t4857: f64, t4954: f64, t4996: f64, t56049: f64, t6244: f64, t6362: f64, t6365: f64, t67501: f64, t67652: f64, t67714: f64, t67927: f64, t78873: f64, t88948: f64) -> f64 {
    let t89725 = -0.15805078039045227836e2_f64 * t67927 * t6365 - 0.79025390195226139183e1_f64 * t67652 * t6365 - 0.15805078039045227836e2_f64 * t43446 * t88948 * t1089 + 0.15805078039045227836e2_f64 * t19526 * t24090 - 0.79025390195226139183e1_f64 * t67714 * t6365 - 0.15805078039045227836e2_f64 * t56049 * t24138 - 0.26341796731742046395e1_f64 * t4857 * t24157 - 0.26341796731742046395e1_f64 * t4996 * t78873 * t24083 + 0.79025390195226139183e1_f64 * t4954 * t24104 + 0.79025390195226139183e1_f64 * t67501 * t6362 + 0.79025390195226139183e1_f64 * t4954 * t24116 + 0.79025390195226139183e1_f64 * t3204 * t19556 * t6244;
    t89725
}
