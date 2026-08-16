//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 917/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk917(t36632: f64, t959: f64, t2660: f64, t36512: f64, t10867: f64, t10972: f64, t1457: f64, t2684: f64, t45423: f64, t7585: f64, t3651: f64, t9972: f64) -> (f64, f64, f64, f64, f64) {
    let t45574 = t36632 * t959;
    let t45575 = 0.14896037479937677779e-1_f64 * t45574;
    let t45577 = 0.25025342966295298669e1_f64 * t36512 * t2660;
    let t45580 = 0.50050685932590597338e1_f64 * t10867 * t1457 * t10972;
    let t45586 = 0.43710935587469654631e2_f64 * t2684 * t7585 * t45423;
    let t45588 = 0.25025342966295298669e1_f64 * t3651 * t9972;
    (t45575, t45577, t45580, t45586, t45588)
}
