//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 726/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk726(t353: f64, t4482: f64, t859: f64, t2242: f64, t894: f64, t2367: f64, t2379: f64, t2233: f64, t2246: f64, t1327: f64, t409: f64, t1285: f64, t1291: f64, t1293: f64, t403: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4483 = t353 * t4482;
    let t4484 = t859 * t4483;
    let t4487 = t2242 * t894;
    let t4489 = t2367 * t2379;
    let t4496 = t2246 * t2233;
    let t4498 = t409 * t1327;
    let t4502 = t1291 * t1285 * t1293 * t403;
    (t4484, t4487, t4489, t4496, t4498, t4502)
}
