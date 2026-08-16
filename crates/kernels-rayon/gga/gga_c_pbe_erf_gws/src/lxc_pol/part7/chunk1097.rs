//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1097/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1097(t19637: f64, t2362: f64, t2382: f64, t2182: f64, t2395: f64, t6148: f64, t830: f64, t1452: f64, t2083: f64, t825: f64, t6154: f64, t6778: f64) -> (f64, f64, f64, f64, f64) {
    let t19639 = t2382 * t19637 * t2362;
    let t19641 = t2395 * t2182;
    let t19643 = t6148 * t830 * t19641;
    let t19646 = t2083 * t1452;
    let t19647 = t19646 * t825;
    let t19652 = t2382 * t6154 * t6778;
    (t19639, t19643, t19646, t19647, t19652)
}
