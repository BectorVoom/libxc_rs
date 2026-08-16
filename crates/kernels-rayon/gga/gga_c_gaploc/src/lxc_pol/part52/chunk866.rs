//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 866/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk866(t2365: f64, t35623: f64, t6111: f64, t35445: f64, t7290: f64, t43581: f64, t35611: f64, t36762: f64, t7785: f64, t44712: f64, t723: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45396 = t6111 * t2365 * t35623;
    let t45397 = 0.29792074959875355558e-1_f64 * t45396;
    let t45407 = t6111 * t2365 * t7290 * t35445;
    let t45408 = 0.17875244975925213335e0_f64 * t45407;
    let t45411 = 0.25561950635947166451e0_f64 * t43581;
    let t45414 = t6111 * t2365 * t35611;
    let t45415 = 0.59584149919750711116e-1_f64 * t45414;
    let t45421 = t36762 * t7785;
    let t45423 = t44712 * t723;
    (t45397, t45408, t45411, t45415, t45421, t45423)
}
