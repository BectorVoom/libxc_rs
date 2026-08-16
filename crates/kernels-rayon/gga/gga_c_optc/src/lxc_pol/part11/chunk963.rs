//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 963/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk963(t3119: f64, t5110: f64, t4336: f64, t16236: f64, t8537: f64, t322: f64, t15240: f64, t5324: f64, t17352: f64, t3245: f64, t17344: f64, t4289: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17662 = t3119 * t5110;
    let t17663 = t4336 * t17662;
    let t17666 = t8537 * t16236;
    let t17667 = t322 * t17666;
    let t17670 = t15240 * t5324;
    let t17674 = t3245 * t17352;
    let t17677 = t4289 * t17344;
    (t17662, t17663, t17666, t17667, t17670, t17674, t17677)
}
