//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 808/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk808(t1969: f64, t32946: f64, t379: f64, t5899: f64, t558: f64, t7339: f64, t2112: f64, t1369: f64, t28: f64, t32869: f64, t586: f64, t375: f64, t7382: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32948 = t1969 * t32946 * t379;
    let t32949 = t5899 * t32948;
    let t32951 = t7339 * t558;
    let t32952 = t2112 * t32951;
    let t32954 = t1369 * t28 * t32952;
    let t32955 = t586 * t32869;
    let t32957 = t1369 * t28 * t32955;
    let t32960 = t89 * t375 * t7382;
    (t32948, t32949, t32951, t32952, t32954, t32955, t32957, t32960)
}
