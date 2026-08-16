//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1052/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1052(t12240: f64, t6977: f64, t3851: f64, t3901: f64, t1337: f64, t562: f64, t3792: f64, t550: f64, t12177: f64, t3897: f64, t1338: f64, t3879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12241 = t6977 * t12240;
    let t12244 = t3901 * t3851;
    let t12247 = t1337 * t1337;
    let t12248 = 1.0_f64 / t12247;
    let t12249 = t12248 * t562;
    let t12250 = t3792 * t550;
    let t12251 = t12177 * t12250;
    let t12252 = t12249 * t12251;
    let t12255 = t12177 * t3792;
    let t12256 = t3897 * t12255;
    let t12259 = t1338 * t3879;
    (t12241, t12244, t12247, t12248, t12249, t12250, t12251, t12252, t12255, t12256, t12259)
}
