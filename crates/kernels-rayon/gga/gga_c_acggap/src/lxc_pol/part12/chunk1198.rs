//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1198/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1198(t35529: f64, t31285: f64, t31287: f64, t31293: f64, t31297: f64, t31299: f64, t31305: f64, t31312: f64, t31316: f64, t31318: f64, t31322: f64, t31342: f64, t31344: f64, t32760: f64, t32763: f64, t32765: f64, t32782: f64, t35535: f64) -> f64 {
    let t37591 = 0.68598428988911579156e-2_f64 * t35529;
    let t37603 = -t32760 - 0.21437009059034868486e-2_f64 * t31285 - 0.13719685797782315831e-1_f64 * t31287 + t32763 - t37591 + t31293 / 16.0_f64 - t32765 - 0.62896184579208304137e-2_f64 * t31297 + 0.56606566121287473724e-1_f64 * t31299 - 0.17149607247227894789e-2_f64 * t35535 - 0.80031500487063509014e-2_f64 * t31305 - 0.17149607247227894789e-2_f64 * t31312 + 0.25158473831683321654e-2_f64 * t31316 + 0.22642626448514989489e-1_f64 * t31318 + 0.17149607247227894789e-2_f64 * t31322 + t32782 + 7.0_f64 / 72.0_f64 * t31342 + 7.0_f64 / 144.0_f64 * t31344;
    t37603
}
