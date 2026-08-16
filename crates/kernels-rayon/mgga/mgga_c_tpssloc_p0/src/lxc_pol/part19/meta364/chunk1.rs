//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1327/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1327(t10523: f64, t41827: f64, t951: f64, t959: f64, t300: f64, t41764: f64, t10853: f64, t2940: f64, t2925: f64, t2951: f64, t2929: f64, t2932: f64, t41733: f64) -> (f64, f64, f64, f64, f64) {
    let t42697 = 0.14035736694323150897e2_f64 * t959 * t10523 * t41827 * t951;
    let t42699 = 0.19751673498613801407e-1_f64 * t300 * t41764;
    let t42701 = 0.20779030926817756511e3_f64 * t2940 * t10853;
    let t42704 = 0.21053605041484726346e2_f64 * t959 * t2951 * t2925;
    let t42708 = 0.51947577317044391277e2_f64 * t959 * t2929 * t41733 * t2932;
    (t42697, t42699, t42701, t42704, t42708)
}
