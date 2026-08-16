//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 929/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk929(t1378: f64, t1985: f64, t23997: f64, t582: f64, t2097: f64, t5935: f64, t53: f64, t925: f64, t3066: f64, t1851: f64, t6454: f64, t2178: f64, t6615: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t107082 = t1985 * t1378;
    let t107284 = t582 * t23997;
    let t107627 = t2097 * t5935;
    let t115418 = t925 * t53;
    let t115567 = t925 * t3066;
    let t117775 = t1851 * t6454;
    let t120449 = t2178 * t6615;
    (t107082, t107284, t107627, t115418, t115567, t117775, t120449)
}
