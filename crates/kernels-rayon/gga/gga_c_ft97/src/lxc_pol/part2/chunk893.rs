//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 893/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk893(t1160: f64, t737: f64, t2609: f64, t13827: f64, t241: f64, t258: f64, t2409: f64, t3897: f64, t2599: f64, t2373: f64, t992: f64, t2600: f64) -> (f64, f64, f64, f64) {
    let t13839 = t737 * t1160;
    let t13840 = t13839 * t2609;
    let t13844 = t241 * t13827 * t258;
    let t13848 = t3897 * t2409;
    let t13849 = t2599 * t13848;
    let t13852 = t992 * t2373;
    let t13853 = t2600 * t13852;
    (t13840, t13844, t13849, t13853)
}
