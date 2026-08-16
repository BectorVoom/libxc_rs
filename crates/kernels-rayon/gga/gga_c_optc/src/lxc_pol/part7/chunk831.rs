//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 831/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk831(t1772: f64, t312: f64, t310: f64, t307: f64, t301: f64, t7312: f64, t300: f64, t2613: f64, t889: f64, t2620: f64, t885: f64, t7192: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7894 = t1772 * t312;
    let t7895 = t310 * t7894;
    let t7897 = 0.80492236016562572729e-3_f64 * t307 * t7895;
    let t7898 = t301 * t7312;
    let t7899 = t300 * t7898;
    let t7902 = t2613 * t889;
    let t7904 = t885 * t2620;
    let t7906 = sigma0 * t7192;
    (t7894, t7895, t7897, t7898, t7899, t7902, t7904, t7906)
}
