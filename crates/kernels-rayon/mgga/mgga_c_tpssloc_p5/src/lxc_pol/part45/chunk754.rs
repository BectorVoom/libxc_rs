//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 754/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk754(t23143: f64, t242: f64, t6612: f64, t812: f64, t2649: f64, t23096: f64, t23100: f64, t23106: f64, t23108: f64, t23114: f64, t23117: f64, t23120: f64, t23125: f64, t23128: f64, t23130: f64, t23135: f64, t23136: f64, t23141: f64) -> (f64, f64) {
    let t23144 = 35.0_f64 / 432.0_f64 * t23143;
    let t23145 = t6612 * t242;
    let t23146 = t812 * t23145;
    let t23147 = t23146 * t2649;
    let t23149 = t23096 + 0.24223653656484234512e-2_f64 * t23100 - t23106 + t23108 + 0.6728792682356731809e-4_f64 * t23114 + t23117 / 1536.0_f64 - t23120 + 0.40372756094140390854e-3_f64 * t23125 - t23128 / 192.0_f64 + 5.0_f64 / 384.0_f64 * t23130 + t23135 - t23136 / 384.0_f64 + t23141 + t23144 + t23147 / 192.0_f64;
    (t23147, t23149)
}
