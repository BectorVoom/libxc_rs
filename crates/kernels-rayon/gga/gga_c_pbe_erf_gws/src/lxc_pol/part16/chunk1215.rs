//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1215/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1215(t14240: f64, t2376: f64, t829: f64, t830: f64, t14327: f64, t2367: f64, t14243: f64, t840: f64, t51869: f64, t1206: f64, t2074: f64, t353: f64, t4386: f64) -> (f64, f64, f64, f64, f64) {
    let t52478 = t2376 * t14240;
    let t52480 = t829 * t830 * t52478;
    let t52483 = t2367 * t14327;
    let t52514 = t840 * t14243;
    let t52525 = 595.0_f64 / 5184.0_f64 * t51869;
    let t52529 = t4386 * t353 * t1206 * t2074;
    (t52480, t52483, t52514, t52525, t52529)
}
