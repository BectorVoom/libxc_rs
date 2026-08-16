//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 909/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk909(t1065: f64, t3143: f64, t137: f64, t420: f64, t125: f64, t3157: f64, t2: f64, t301: f64, t3153: f64, t1149: f64, t986: f64, t1152: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13754 = t3143 * t1065;
    let t13761 = t420 * t137;
    let t13768 = t125 * t3157;
    let t13771 = t3153 * t13768 * t301 * t2;
    let t13787 = t986 * t1149;
    let t13788 = t13787 * t1152;
    (t13754, t13761, t13768, t13771, t13787, t13788)
}
