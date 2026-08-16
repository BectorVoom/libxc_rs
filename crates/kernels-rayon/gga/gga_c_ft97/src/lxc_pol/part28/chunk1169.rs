//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1169/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1169(t35068: f64, t8392: f64, t1053: f64, t2179: f64, t32992: f64, t49562: f64, t7400: f64, t2142: f64, t34947: f64, t1882: f64, t35181: f64, t35217: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t148922 = t8392 * t35068;
    let t148943 = t2179 * t32992 * t1053;
    let t148955 = t49562 * t7400;
    let t148960 = t2142 * t34947;
    let t148964 = t1882 * t35181;
    let t148966 = t1882 * t35217;
    (t148922, t148943, t148955, t148960, t148964, t148966)
}
