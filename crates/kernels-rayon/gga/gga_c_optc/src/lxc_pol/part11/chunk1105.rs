//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1105/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1105(t11899: f64, t5328: f64, t8446: f64, t1150: f64, t5403: f64, t7274: f64, t1113: f64, t5311: f64, t3017: f64, t5165: f64, t1782: f64, t5145: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44001 = t11899 * t5328;
    let t44014 = t8446 * t5328;
    let t44077 = t1150 * t7274 * t5403;
    let t44090 = t1113 * t5311;
    let t44181 = t5165 * t3017;
    let t44193 = t1782 * t5145;
    (t44001, t44014, t44077, t44090, t44181, t44193)
}
