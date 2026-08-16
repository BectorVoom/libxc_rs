//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1720/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1720<F: Float>(t1089: F, t19526: F, t19556: F, t24083: F, t24090: F, t24104: F, t24116: F, t24138: F, t24157: F, t3204: F, t43446: F, t4857: F, t4954: F, t4996: F, t56049: F, t6244: F, t6362: F, t6365: F, t67501: F, t67652: F, t67714: F, t67927: F, t78873: F, t88948: F) -> F {
    let t89725 = -F::cast_from(0.15805078039045227836e2_f64) * t67927 * t6365 - F::cast_from(0.79025390195226139183e1_f64) * t67652 * t6365 - F::cast_from(0.15805078039045227836e2_f64) * t43446 * t88948 * t1089 + F::cast_from(0.15805078039045227836e2_f64) * t19526 * t24090 - F::cast_from(0.79025390195226139183e1_f64) * t67714 * t6365 - F::cast_from(0.15805078039045227836e2_f64) * t56049 * t24138 - F::cast_from(0.26341796731742046395e1_f64) * t4857 * t24157 - F::cast_from(0.26341796731742046395e1_f64) * t4996 * t78873 * t24083 + F::cast_from(0.79025390195226139183e1_f64) * t4954 * t24104 + F::cast_from(0.79025390195226139183e1_f64) * t67501 * t6362 + F::cast_from(0.79025390195226139183e1_f64) * t4954 * t24116 + F::cast_from(0.79025390195226139183e1_f64) * t3204 * t19556 * t6244;
    t89725
}
