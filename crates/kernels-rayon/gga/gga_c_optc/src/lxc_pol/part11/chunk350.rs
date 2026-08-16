//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 350/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk350(t1544: f64, t155: f64, t1150: f64, t1159: f64, t1162: f64, t1170: f64, t1177: f64, t1179: f64, t1520: f64, t1529: f64, t1533: f64, t1536: f64, t1541: f64, t451: f64, t459: f64) -> (f64, f64) {
    let t1545 = t155 * t1544;
    let t1550 = 0.11360101276506094136e1_f64 * t1150 * t1529 - 0.23181763972770020946e0_f64 * t1533 * t459 + t1159 + 0.28977204965962526182e-1_f64 * t1162 * t1536 + 0.5848048239485271795e1_f64 * t1170 * t1541 - 0.4030456356669135783e-1_f64 * t1545 * t451 + t1177 + 0.50380704458364197288e-2_f64 * t1179 * t1520;
    (t1545, t1550)
}
