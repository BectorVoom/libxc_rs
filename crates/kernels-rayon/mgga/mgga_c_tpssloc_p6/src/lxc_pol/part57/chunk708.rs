//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 708/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk708(t225: f64, t3173: f64, t368: f64, t3068: f64, t1058: f64, t1926: f64, t3158: f64, t1942: f64, t3082: f64, t344: f64, t40: f64, t1009: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t23394 = t225 * t3173;
    let t23417 = sigma0 * t368;
    let t23418 = t23417 * t3068;
    let t23419 = t1058 * t23418;
    let t23447 = t1926 * t3158 / 432.0_f64;
    let t23469 = t1942 * t3082 / 6912.0_f64;
    let t23470 = t40 * t344;
    let t23471 = t23470 * t1009;
    (t23394, t23419, t23447, t23469, t23471)
}
