//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1213/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1213(t14202: f64, t4414: f64, t14340: f64, t9270: f64, t14286: f64, t840: f64, t1205: f64, t19631: f64, t829: f64, t830: f64, t4083: f64, t4424: f64) -> (f64, f64, f64, f64, f64) {
    let t52309 = t4414 * t14202;
    let t52331 = t9270 * t14340;
    let t52345 = t840 * t14286;
    let t52348 = t19631 * t1205;
    let t52350 = t829 * t830 * t52348;
    let t52353 = t4424 * t4083;
    (t52309, t52331, t52345, t52350, t52353)
}
