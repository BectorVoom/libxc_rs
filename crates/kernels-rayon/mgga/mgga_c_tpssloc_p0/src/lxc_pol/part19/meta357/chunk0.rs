//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1294/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1294(t2853: f64, t2885: f64, t10523: f64, t938: f64, t10660: f64, t888: f64, t10663: f64, t10702: f64, t2844: f64, t41995: f64, t10810: f64, t919: f64) -> (f64, f64, f64, f64, f64) {
    let t42123 = t2853 * t2885;
    let t42128 = t938 * t10523;
    let t42143 = t888 * t10660;
    let t42145 = 0.3859675079686208416e3_f64 * t42143 * t10663;
    let t42148 = 0.57895126195293126241e3_f64 * t10702 * t41995 * t2844;
    let t42149 = t919 * t10810;
    (t42123, t42128, t42145, t42148, t42149)
}
