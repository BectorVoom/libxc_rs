//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1068/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1068(t8628: f64, t8667: f64, t8721: f64, t8772: f64, t9211: f64, t9273: f64, t9320: f64, t9737: f64, t4383: f64, t6158: f64, t1114: f64, t3222: f64, t9607: f64) -> (f64, f64, f64) {
    let t9740 = t8628 + t8667 + t8721 + t8772 + t9211 + t9273 + t9320 + t9737;
    let t11374 = t6158 * t4383;
    let t11375 = t1114 * t11374;
    let t11434 = t9607 * t3222;
    (t9740, t11375, t11434)
}
