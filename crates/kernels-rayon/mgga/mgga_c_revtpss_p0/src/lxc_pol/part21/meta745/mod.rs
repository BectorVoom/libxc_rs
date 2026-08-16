//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta745 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta745(t3857: f64, t5567: f64, t1317: f64, t13672: f64, t2608: f64, t512: f64, t5566: f64, t1856: f64, t9544: f64, t46975: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t46970: f64, t48223: f64, t48224: f64, t48226: f64, t48228: f64, t48231: f64, t48232: f64, t48234: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t48236, t48238, t48241, t48243, t48244, t48245) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2619(t3857, t5567, t1317, t13672, t2608, t512, t5566, t1856, t9544, t46975, t39483, t39520, t39528, t39531, t46970, t48223, t48224, t48226, t48228, t48231, t48232, t48234);
    (t48236, t48238, t48241, t48243, t48244, t48245)
}
