//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1255/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1255(t1112: f64, t13918: f64, t361: f64, t13917: f64, t6639: f64, t14424: f64, t9381: f64, t353: f64, t4183: f64, t814: f64, t859: f64, t52915: f64, t9521: f64) -> (f64, f64, f64, f64, f64) {
    let t53446 = t13918 * t1112;
    let t53447 = t361 * t53446;
    let t53449 = t13917 * t53447 * t6639;
    let t53460 = t13917 * t14424 * t9381;
    let t53464 = t859 * t353 * t4183 * t814;
    let t53468 = t13917 * t52915 * t9521;
    (t53447, t53449, t53460, t53464, t53468)
}
