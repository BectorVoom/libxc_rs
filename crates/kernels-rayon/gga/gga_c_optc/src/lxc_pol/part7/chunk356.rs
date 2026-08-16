//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 356/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk356(t1138: f64, t1150: f64, t1153: f64, t1159: f64, t1162: f64, t1163: f64, t1170: f64, t1173: f64, t1177: f64, t1179: f64) -> f64 {
    let t1182 = 0.11360101276506094136e1_f64 * t1150 * t1153 + t1159 + 0.28977204965962526182e-1_f64 * t1162 * t1163 + 0.5848048239485271795e1_f64 * t1170 * t1173 + t1177 + 0.50380704458364197288e-2_f64 * t1179 * t1138;
    t1182
}
