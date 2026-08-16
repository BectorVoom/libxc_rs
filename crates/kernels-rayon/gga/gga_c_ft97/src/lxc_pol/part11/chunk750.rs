//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 750/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk750(t1882: f64, t2587: f64, t2409: f64, t724: f64, t773: f64, t2614: f64, t2581: f64, t2469: f64, t2526: f64, t242: f64, t2542: f64, t761: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10140 = t1882 * t2587;
    let t10143 = t724 * t773 * t2409;
    let t10146 = t1882 * t2614;
    let t10148 = t1882 * t2581;
    let t10150 = t2469 * t2526;
    let t10151 = t242 * t10150;
    let t10153 = t2542 * t761;
    (t10140, t10143, t10146, t10148, t10150, t10151, t10153)
}
