//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1299/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1299(t1123: f64, t2848: f64, t331: f64, t833: f64, t850: f64, t11576: f64, t14423: f64, t14682: f64, t3989: f64, t13859: f64, t56296: f64, t6287: f64) -> (f64, f64, f64) {
    let t56578 = t850 * t1123 * t2848 * t331 * t833;
    let t56582 = t3989 * t14682 * t14423 * t11576;
    let t56586 = t13859 * t14682 * t56296 * t6287;
    (t56578, t56582, t56586)
}
