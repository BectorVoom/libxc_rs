//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1786/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1786(t23146: f64, t2649: f64, t23096: f64, t23100: f64, t23106: f64, t23108: f64, t23114: f64, t23117: f64, t23120: f64, t23125: f64, t23128: f64, t23130: f64, t23135: f64, t23136: f64, t23141: f64, t23144: f64) -> f64 {
    let t23147 = t23146 * t2649;
    let t23149 = t23096 + 0.24223653656484234512e-2_f64 * t23100 - t23106 + t23108 + 0.6728792682356731809e-4_f64 * t23114 + t23117 / 1536.0_f64 - t23120 + 0.40372756094140390854e-3_f64 * t23125 - t23128 / 192.0_f64 + 5.0_f64 / 384.0_f64 * t23130 + t23135 - t23136 / 384.0_f64 + t23141 + t23144 + t23147 / 192.0_f64;
    t23149
}
