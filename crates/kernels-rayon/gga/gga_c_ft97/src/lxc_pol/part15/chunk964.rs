//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 964/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk964(t172: f64, t21210: f64, t228: f64, t231: f64, t17868: f64, t2393: f64, t3771: f64, t236: f64, t2378: f64, t37481: f64, t1690: f64, t5266: f64) -> (f64, f64, f64, f64) {
    let t79809 = t228 * t21210 * t172 * t231;
    let t79892 = t3771 * t17868 * t2393;
    let t79950 = t3771 * t236 * t37481 * t2378;
    let t79951 = t1690 * t5266;
    (t79809, t79892, t79950, t79951)
}
