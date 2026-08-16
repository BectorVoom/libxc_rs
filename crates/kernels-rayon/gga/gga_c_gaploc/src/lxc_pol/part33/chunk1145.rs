//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1145/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1145(t20957: f64, t9294: f64, t18823: f64, t2476: f64, t9438: f64, t1457: f64, t6655: f64, t10525: f64, t20370: f64, t2365: f64, t30110: f64, t900: f64, t9561: f64) -> (f64, f64, f64, f64, f64) {
    let t30835 = 0.59584149919750711116e-1_f64 * t20957 * t9294;
    let t30843 = t2476 * t9438 * t18823;
    let t30848 = t1457 * t6655;
    let t30897 = 0.17875244975925213335e0_f64 * t10525 * t2365 * t20370;
    let t30900 = 0.20854452471912748891e0_f64 * t9561 * t900 * t30110;
    (t30835, t30843, t30848, t30897, t30900)
}
