//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 599/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk599(t1306: f64, t135: f64, t273: f64, t3032: f64, t3035: f64, t3037: f64, t3040: f64, t3072: f64, t3076: f64, t3144: f64, t3146: f64, t3149: f64, t3151: f64, t3155: f64, t3159: f64, t3164: f64, t3282: f64, t3286: f64, t955: f64, t957: f64) -> f64 {
    let t3289 = t135 * t273 * t3282 * t957 - t1306 * t3286 * t955 - t3032 + t3035 + t3037 - t3040 + t3072 + t3076 + t3144 + t3146 - t3149 - t3151 + t3155 - t3159 - t3164;
    t3289
}
