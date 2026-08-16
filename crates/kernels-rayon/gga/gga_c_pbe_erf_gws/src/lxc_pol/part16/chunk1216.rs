//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1216/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1216(t353: f64, t4111: f64, t814: f64, t859: f64, t14180: f64, t4386: f64, t892: f64, t14280: f64, t840: f64, t2242: f64, t4094: f64, t4083: f64, t4453: f64) -> (f64, f64, f64, f64, f64) {
    let t52534 = t859 * t353 * t4111 * t814;
    let t52542 = t4386 * t892 * t14180;
    let t52551 = t840 * t14280;
    let t52560 = t2242 * t4094;
    let t52562 = t4453 * t4083;
    (t52534, t52542, t52551, t52560, t52562)
}
