//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 652/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk652(t252: f64, t4142: f64, t1492: f64, t852: f64, t1493: f64, t225: f64, t1519: f64, t798: f64, t1496: f64, t2563: f64, t1495: f64, t210: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4143 = t4142 * t252;
    let t4145 = t1492 * t852;
    let t4147 = t1493 * t225;
    let t4149 = t798 * t1519;
    let t4152 = t2563 * t1496;
    let t4155 = t210 * t1495 * t776;
    (t4143, t4145, t4147, t4149, t4152, t4155)
}
