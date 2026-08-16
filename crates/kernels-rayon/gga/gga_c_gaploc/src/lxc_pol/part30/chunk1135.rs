//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1135/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1135(t1328: f64, t587: f64, t9438: f64, t9439: f64, t2487: f64, t9448: f64, t4379: f64, t9580: f64, t21077: f64, t901: f64, t2372: f64, t6625: f64) -> (f64, f64, f64, f64, f64) {
    let t30326 = t587 * t9438 * t9439 * t1328;
    let t30330 = t2487 * t9438 * t9448 * t1328;
    let t30339 = 0.11916829983950142223e0_f64 * t4379 * t9580;
    let t30354 = 0.59584149919750711116e-1_f64 * t21077 * t901;
    let t30356 = 0.17875244975925213335e0_f64 * t2372 * t6625;
    (t30326, t30330, t30339, t30354, t30356)
}
