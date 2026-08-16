//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 907/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk907(t1: f64, t17045: f64, t297: f64, t313: f64, t16988: f64, t7380: f64, t935: f64, t16225: f64, t7405: f64, t322: f64, t7924: f64, t16231: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17047 = t17045 * t1 * t297;
    let t17048 = t313 * t17047;
    let t17052 = t16988 * t7380 * t935;
    let t17053 = t313 * t17052;
    let t17056 = t7405 * t16225;
    let t17057 = t322 * t17056;
    let t17060 = t7924 * t16225;
    let t17061 = t322 * t17060;
    let t17064 = t865 * t16231;
    (t17047, t17048, t17052, t17053, t17056, t17057, t17060, t17061, t17064)
}
