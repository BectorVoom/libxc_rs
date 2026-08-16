//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2657/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2657(t13937: f64, t9962: f64, t13991: f64, t13999: f64, t13786: f64, t13760: f64, t9765: f64, t13756: f64, t3989: f64, t268: f64, t5617: f64, t46784: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48892 = t9962 * t13937;
    let t48900 = t13999 * t13991;
    let t48902 = t9962 * t13786;
    let t48904 = t9765 * t13760;
    let t48905 = 0.16262400898971305032e-2_f64 * t48904;
    let t48906 = t3989 * t13756;
    let t48908 = t5617 * t268;
    let t48909 = t46784 * t48908;
    (t48892, t48900, t48902, t48905, t48906, t48908, t48909)
}
