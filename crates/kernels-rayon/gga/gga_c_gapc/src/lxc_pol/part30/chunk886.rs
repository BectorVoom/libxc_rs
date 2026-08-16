//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 886/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk886(t9097: f64, t9100: f64, t9104: f64, t9106: f64, t9108: f64, t9111: f64, t9115: f64, t9118: f64, t9121: f64, t9124: f64, t9126: f64, t9130: f64, t9132: f64) -> f64 {
    let t10708 = -0.33816362383187442026e-4_f64 * t9097 + 0.28985453471303521736e-5_f64 * t9100 - 0.91551759647971344971e-6_f64 * t9104 + 0.33816362383187442026e-4_f64 * t9106 - 0.10136107947527008247e-3_f64 * t9108 - 0.10136107947527008247e-3_f64 * t9111 - 0.37516872880543120646e-8_f64 * t9115 + 0.25294579912893309636e-8_f64 * t9118 + 0.12974218172834570556e-1_f64 * t9121 - 0.27801896084645508334e-2_f64 * t9124 + 0.132681342766433194e-5_f64 * t9126 + 0.20241536458333333336e-3_f64 * t9130 + 0.55603792169291016668e-2_f64 * t9132;
    t10708
}
