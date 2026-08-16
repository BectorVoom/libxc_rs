//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1209/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1209(t241: f64, t24748: f64, t24887: f64, t24917: f64, t24972: f64, t24708: f64, t24712: f64, t24715: f64, t24718: f64, t24721: f64, t24723: f64, t24955: f64, t24957: f64, t24960: f64, t24964: f64, t24968: f64) -> (f64, f64) {
    let t24975 = t241 * (t24748 + t24887 + t24917 + t24972);
    let t24976 = t24708 + t24712 - t24715 - t24718 - t24721 - t24723 + t24975 - t24955 + t24957 - t24960 + t24964 + t24968;
    (t24975, t24976)
}
