//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 815/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk815(t2160: f64, t6542: f64, t2289: f64, t2293: f64, t2262: f64, t344: f64, t362: f64, t2209: f64, t825: f64, t2182: f64, t337: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6543 = t6542 * t2160;
    let t6545 = t2289 * t2293;
    let t6552 = 1.0_f64 / t2262 / t344;
    let t6553 = t6552 * t362;
    let t6560 = t825 * t2209;
    let t6562 = t337 * t5 * t2182;
    (t6543, t6545, t6552, t6553, t6560, t6562)
}
