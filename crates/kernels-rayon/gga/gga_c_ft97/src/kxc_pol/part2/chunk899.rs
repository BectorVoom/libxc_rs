//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 899/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk899(t13928: f64, t242: f64, t3894: f64, t8392: f64, t2413: f64, t3869: f64, t2606: f64, t2405: f64, t3891: f64, t3972: f64, t761: f64, t684: f64) -> (f64, f64, f64, f64, f64) {
    let t13929 = t242 * t13928;
    let t13933 = 4.0_f64 / 81.0_f64 * t8392 * t3894;
    let t13934 = t3869 * t2413;
    let t13935 = t2606 * t13934;
    let t13938 = t3869 * t2405;
    let t13939 = t3891 * t13938;
    let t13942 = t761 * t3972;
    let t13943 = t13942 * t684;
    (t13929, t13933, t13935, t13939, t13943)
}
