//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2462/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2462(t3057: f64, t3316: f64, t4891: f64, t3298: f64, t3059: f64, t3154: f64, t1045: f64, t2853: f64, t999: f64, t11774: f64, t127: f64, t3096: f64, t3128: f64) -> (f64, f64, f64, f64, f64) {
    let t43043 = t3057 * t3316;
    let t43044 = t43043 * t4891;
    let t43049 = t3057 * t3298;
    let t43050 = t43049 * t4891;
    let t43051 = t3154 * t3059;
    let t43057 = t1045 * t2853 * t999;
    let t43063 = t11774 * t127 * t3128 * t3096;
    (t43044, t43050, t43051, t43057, t43063)
}
