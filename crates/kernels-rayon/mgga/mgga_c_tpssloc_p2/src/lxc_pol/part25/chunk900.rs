//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 900/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk900(t11557: f64, t1174: f64, t135: f64, t3471: f64, t11168: f64, t4908: f64, t11159: f64, t4900: f64, t1184: f64, t4899: f64, t3242: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11558 = t1174 * t11557;
    let t11560 = t135 * t3471;
    let t11561 = t1174 * t11560;
    let t11563 = t4908 * t11168;
    let t11566 = t4900 * t11159;
    let t11569 = t4899 * t1184;
    let t11570 = t460 * t3242;
    (t11558, t11561, t11563, t11566, t11569, t11570)
}
