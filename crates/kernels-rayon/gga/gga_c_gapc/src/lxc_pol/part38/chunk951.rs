//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 951/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk951(t11371: f64, t11461: f64, t11531: f64, t11607: f64, t576: f64, t932: f64, t996: f64, t3723: f64, t787: f64, t876: f64, t1054: f64, t125: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11609 = t11371 + t11461 + t11531 + t11607;
    let t11610 = t576 * t11609;
    let t11612 = t996 * t932;
    let t11613 = t3723 * t787;
    let t11614 = t11612 * t11613;
    let t11616 = t3723 * t876;
    let t11617 = t1054 * t11616;
    let t11619 = t825 * t125;
    (t11609, t11610, t11612, t11613, t11614, t11616, t11617, t11619)
}
