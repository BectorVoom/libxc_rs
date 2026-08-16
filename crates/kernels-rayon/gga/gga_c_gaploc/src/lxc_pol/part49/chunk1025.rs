//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1025/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1025(t1022: f64, t9755: f64, t2639: f64, t787: f64, t28002: f64, t9858: f64, t13141: f64, t2464: f64, t2684: f64, t43007: f64, t701: f64, t6066: f64, t7630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43572 = t9755 * t1022;
    let t43575 = 0.53625734927775640005e1_f64 * t787 * t43572 * t2639;
    let t43579 = 0.17875244975925213335e2_f64 * t787 * t28002 * t1022 * t9858;
    let t43581 = t2684 * t2464 * t13141;
    let t43582 = 0.63904876589867916128e-1_f64 * t43581;
    let t43586 = t43007 * t701;
    let t43588 = t7630 * t6066 * t43586;
    (t43572, t43575, t43579, t43582, t43586, t43588)
}
