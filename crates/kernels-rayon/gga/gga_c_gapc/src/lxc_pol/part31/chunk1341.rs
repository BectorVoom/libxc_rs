//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1341/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1341(t24352: f64, t2920: f64, t35894: f64, t10105: f64, t3724: f64, t10343: f64, t11695: f64, t12049: f64, t12056: f64, t30523: f64, t8610: f64, t12052: f64, t23726: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36040 = t2920 * t24352 * t35894;
    let t36042 = t10105 * t3724;
    let t36044 = t10343 * t11695;
    let t36092 = 2.0_f64 * t12049;
    let t36093 = 4.0_f64 * t12056;
    let t36095 = 6.0_f64 * t30523 * t8610;
    let t36100 = 12.0_f64 * t23726 * t12052;
    (t36040, t36042, t36044, t36092, t36093, t36095, t36100)
}
