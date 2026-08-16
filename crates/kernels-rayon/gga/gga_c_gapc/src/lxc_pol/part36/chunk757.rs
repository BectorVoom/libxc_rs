//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 757/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk757(t1749: f64, t9023: f64, t3132: f64, t5285: f64, t1881: f64, t512: f64, t178: f64, t173: f64, t7216: f64, t1027: f64, t1787: f64, t1740: f64, t9016: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9024 = t9023 * t1749;
    let t9026 = t5285 * t3132;
    let t9027 = t9026 * t1749;
    let t9029 = t1881 * t512;
    let t9030 = t178 * t9029;
    let t9031 = t173 * t7216;
    let t9032 = t9030 * t9031;
    let t9034 = t1027 * t1787;
    let t9036 = t9016 * t1740;
    (t9024, t9027, t9029, t9032, t9034, t9036)
}
