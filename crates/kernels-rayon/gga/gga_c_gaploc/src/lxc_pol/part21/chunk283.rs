//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 283/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk283(t338: f64, t13: f64, t30: f64, t1135: f64) -> f64 {
    let t1154 = t338 * t338;
    let t1155 = 1.0_f64 / t1154;
    let t1156 = t13 * t1155;
    let t1157 = t30 * t30;
    let t1158 = 1.0_f64 / t1157;
    let t1159 = t1135 * t1158;
    let t1161 = 0.16081824322151104822e2_f64 * t1156 * t1159;
    t1161
}
