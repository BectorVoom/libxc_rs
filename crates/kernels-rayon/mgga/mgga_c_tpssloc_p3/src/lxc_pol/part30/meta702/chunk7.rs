//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2280/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2280(t17667: f64, t23537: f64, t1622: f64, t17925: f64, t17962: f64, t23529: f64, t5861: f64, t5875: f64, t5880: f64, t6755: f64, t82848: f64, t82851: f64, t82956: f64, t83043: f64, t83061: f64, t83215: f64, t88249: f64, t88584: f64) -> f64 {
    let t99483 = t23537 * t17667;
    let t99492 = -t83215 * t17925 / 1152.0_f64 + t6755 * t17962 / 1536.0_f64 + t83043 * t5875 / 768.0_f64 - t83061 * t5880 / 1536.0_f64 - t82956 * t5875 / 144.0_f64 + t99483 / 1152.0_f64 + t82848 * t5880 / 288.0_f64 - t82851 / 6912.0_f64 - t88584 * t1622 / 216.0_f64 - 5.0_f64 / 1296.0_f64 * t23529 * t5861 + t88249;
    t99492
}
