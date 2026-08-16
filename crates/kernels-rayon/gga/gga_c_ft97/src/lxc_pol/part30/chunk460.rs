//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 460/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk460(t263: f64, t7485: f64, t193: f64, t1425: f64, t1454: f64, t2574: f64, t265: f64, t7440: f64, t1424: f64, t1456: f64, t729: f64, t1449: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7486 = t7485 * t263;
    let t7487 = t193 * t7486;
    let t7490 = t1425 * t1454;
    let t7491 = t193 * t7490;
    let t7495 = t2574 * t265 * t7440;
    let t7499 = t729 * t1456 * t1424;
    let t7502 = t1424 * t1449;
    (t7486, t7487, t7490, t7491, t7495, t7499, t7502)
}
