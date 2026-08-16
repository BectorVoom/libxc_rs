//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1324/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1324(t11938: f64, t11944: f64, t11954: f64, t1670: f64, t1674: f64, t1675: f64, t1713: f64, t20040: f64, t20043: f64, t20046: f64, t20048: f64, t20049: f64, t20052: f64, t2853: f64, t4099: f64, t4822: f64, t96: f64) -> f64 {
    let t24654 = 12.0_f64 * t1670 * t1674 * t4822 + 12.0_f64 * t1674 * t1675 * t4099 + 6.0_f64 * t1713 * t2853 * t96 - t11938 - t11944 - t11954 - t20040 - t20043 - t20046 - t20048 - t20049 + t20052;
    t24654
}
