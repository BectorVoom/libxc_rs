//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1269/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1269(t1890: f64, t9879: f64, t9874: f64, t7942: f64, t9883: f64, t9906: f64, t9901: f64, t9890: f64, t9894: f64, t23313: f64, t23315: f64, t23317: f64, t23319: f64, t23321: f64) -> f64 {
    let t27294 = t1890 * t9879;
    let t27296 = t1890 * t9874;
    let t27298 = t7942 * t9883;
    let t27300 = t1890 * t9906;
    let t27302 = t1890 * t9901;
    let t27304 = t1890 * t9890;
    let t27306 = t7942 * t9894;
    let t27308 = 10.0_f64 / 729.0_f64 * t23313 + 8.0_f64 / 243.0_f64 * t23315 - 2.0_f64 / 81.0_f64 * t23317 - 8.0_f64 / 81.0_f64 * t23319 + 16.0_f64 / 243.0_f64 * t23321 - 4.0_f64 / 81.0_f64 * t27294 + 10.0_f64 / 729.0_f64 * t27296 + 44.0_f64 / 243.0_f64 * t27298 - 2.0_f64 / 81.0_f64 * t27300 + 2.0_f64 / 243.0_f64 * t27302 + 2.0_f64 / 27.0_f64 * t27304 - 44.0_f64 / 81.0_f64 * t27306;
    t27308
}
