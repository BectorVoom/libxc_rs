//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1027/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1027(t25985: f64, t32333: f64, t22952: f64, t22958: f64, t136151: f64, t136159: f64, t1317: f64, t34416: f64, t376: f64, t3157: f64, t7165: f64, t1800: f64, t28: f64, t5665: f64) -> (f64, f64, f64, f64, f64) {
    let t144928 = t32333 * t25985;
    let t144930 = t22952 * t22958 * t144928;
    let t144933 = t136159 * t136151 * t144928;
    let t144935 = t1317 * t376 * t34416;
    let t144938 = t7165 * t3157;
    let t144941 = t5665 * t28 * t1800 * t144938;
    (t144930, t144933, t144935, t144938, t144941)
}
