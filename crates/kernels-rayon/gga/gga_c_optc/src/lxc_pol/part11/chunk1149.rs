//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1149/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1149(t50765: f64, t893: f64, t10856: f64, t16961: f64, t2668: f64, t10894: f64, t16984: f64, t2640: f64, t16644: f64, t8152: f64, t862: f64, t16990: f64, t7386: f64, t888: f64) -> (f64, f64, f64, f64, f64) {
    let t50766 = t893 * t50765;
    let t50823 = t2668 * t10856 * t16961;
    let t50828 = t2640 * t10894 * t16984;
    let t50869 = t862 * t8152 * t16644;
    let t50874 = t7386 * t888 * t16990;
    (t50766, t50823, t50828, t50869, t50874)
}
