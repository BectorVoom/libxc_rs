//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta744 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2618;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta744(t13581: f64, t177: f64, t762: f64, t46971: f64, t1317: f64, t13632: f64, t3857: f64, t5569: f64, t512: f64, t749: f64, t46973: f64, t3863: f64, t5567: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t48223, t48224, t48226, t48228, t48231, t48232, t48234) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2618(t13581, t177, t762, t46971, t1317, t13632, t3857, t5569, t512, t749, t46973, t3863, t5567);
    (t48223, t48224, t48226, t48228, t48231, t48232, t48234)
}
