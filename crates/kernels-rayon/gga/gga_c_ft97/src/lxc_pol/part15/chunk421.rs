//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 421/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk421(t1170: f64, t1882: f64, t1144: f64, t1186: f64, t2336: f64, t89: f64, t1213: f64, t375: f64, t1212: f64, t2680: f64, t1196: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3986 = t1882 * t1170;
    let t3988 = t1882 * t1144;
    let t4032 = t89 * t2336 * t1186;
    let t4049 = t89 * t375 * t1213;
    let t4056 = t2680 * t1212;
    let t4064 = t816 * t1196;
    (t3986, t3988, t4032, t4049, t4056, t4064)
}
