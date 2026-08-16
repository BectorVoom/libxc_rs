//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 820/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk820(t3061: f64, t5218: f64, t5202: f64, t8700: f64, t2976: f64, t5154: f64, t5122: f64, t8850: f64, t1085: f64, t5197: f64, t1066: f64, t5117: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15374 = t5218 * t3061;
    let t15381 = t5202 * t8700;
    let t15401 = t5154 * t2976;
    let t15408 = t5122 * t8850;
    let t15434 = t5197 * t1085;
    let t15496 = t5117 * t1066;
    (t15374, t15381, t15401, t15408, t15434, t15496)
}
