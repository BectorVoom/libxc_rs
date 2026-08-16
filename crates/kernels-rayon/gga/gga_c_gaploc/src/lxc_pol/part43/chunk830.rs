//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 830/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk830(t35216: f64, t9287: f64, t2875: f64, t4386: f64, t544: f64, t9078: f64, t2792: f64, t3177: f64, t9263: f64, t9278: f64, t993: f64, t20671: f64, t31041: f64, t34818: f64) -> (f64, f64, f64, f64, f64) {
    let t41676 = t35216 * t9287;
    let t41677 = 0.29792074959875355558e-1_f64 * t41676;
    let t41681 = 0.27805936629216998521e0_f64 * t544 * t9078 * t2875 * t4386;
    let t41683 = t9263 * t2792 * t3177;
    let t41686 = t9263 * t993 * t9278;
    let t41689 = t31041 * t20671 * t34818;
    (t41677, t41681, t41683, t41686, t41689)
}
