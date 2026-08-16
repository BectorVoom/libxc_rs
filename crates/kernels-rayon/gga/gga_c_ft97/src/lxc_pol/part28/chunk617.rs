//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 617/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk617(t23031: f64, t3188: f64, t22953: f64, t5674: f64, t1642: f64, t469: f64, t22986: f64, t379: f64, t6469: f64, t22958: f64, t375: f64, t6520: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25924 = t23031 * t3188;
    let t25925 = t22953 * t25924;
    let t25926 = t5674 * t25925;
    let t25928 = t1642 * t469;
    let t25929 = t22986 * t3188;
    let t25930 = t25928 * t25929;
    let t25931 = t5674 * t25930;
    let t25933 = t6469 * t379;
    let t25934 = t22958 * t25933;
    let t25935 = t5674 * t25934;
    let t25940 = t89 * t375 * t6520;
    (t25924, t25926, t25928, t25929, t25931, t25933, t25935, t25940)
}
