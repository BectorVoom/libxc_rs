//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 680/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk680(t26883: f64, t574: f64, t605: f64, t1359: f64, t3408: f64, t167: f64, t2185: f64, t1060: f64, t5860: f64, t558: f64, t6718: f64, t144: f64, t26521: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26885 = t574 * t605 * t26883;
    let t26888 = t1359 * t3408;
    let t26890 = t2185 * t167 * t26888;
    let t26894 = t2185 * t1060 * t5860;
    let t26897 = t6718 * t558;
    let t26899 = t574 * t605 * t26897;
    let t26902 = t144 * t26521;
    (t26885, t26888, t26890, t26894, t26897, t26899, t26902)
}
