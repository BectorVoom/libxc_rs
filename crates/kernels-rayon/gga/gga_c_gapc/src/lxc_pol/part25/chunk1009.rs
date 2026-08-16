//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1009/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1009(t11625: f64, t11626: f64, t2212: f64, t2268: f64, t3738: f64, t10346: f64, t2208: f64, t6201: f64, t800: f64, t3649: f64, t760: f64, t3739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11627 = t11625 * t11626;
    let t11629 = t2268 * t2212;
    let t11630 = t3738 * t11629;
    let t11632 = t10346 * t2208;
    let t11633 = t800 * t6201;
    let t11634 = t11632 * t11633;
    let t11636 = t3649 * t760;
    let t11637 = t11636 * t2208;
    let t11638 = t11637 * t3739;
    (t11627, t11629, t11630, t11632, t11633, t11634, t11636, t11637, t11638)
}
