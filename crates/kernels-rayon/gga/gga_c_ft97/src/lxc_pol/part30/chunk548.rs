//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 548/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk548(t6140: f64, t681: f64, t89: f64, t1434: f64, t6128: f64, t1424: f64, t2347: f64, t1882: f64, t6137: f64, t6061: f64, t668: f64, t2360: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24499 = t681 * t6140;
    let t24500 = t89 * t24499;
    let t24517 = t1434 * t681 * t6128;
    let t24519 = t1424 * t2347;
    let t24524 = t1882 * t6137;
    let t24526 = t6061 * t668;
    let t24531 = t1424 * t2360;
    (t24499, t24500, t24517, t24519, t24524, t24526, t24531)
}
