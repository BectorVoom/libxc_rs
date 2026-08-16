//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 528/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk528(t3177: f64, t6985: f64, t2487: f64, t589: f64, t587: f64, t2365: f64, t6510: f64, t4391: f64, t544: f64, t6851: f64, t2326: f64, t900: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9552 = t6985 * t3177;
    let t9553 = t2487 * t9552;
    let t9554 = 0.51123901271894332901e0_f64 * t9553;
    let t9555 = t589 * t3177;
    let t9556 = t587 * t9555;
    let t9557 = 0.51123901271894332901e0_f64 * t9556;
    let t9558 = t2365 * t6510;
    let t9560 = 0.59584149919750711116e-1_f64 * t4391 * t9558;
    let t9561 = t544 * t6851;
    let t9562 = t900 * t2326;
    (t9553, t9554, t9556, t9557, t9560, t9561, t9562)
}
