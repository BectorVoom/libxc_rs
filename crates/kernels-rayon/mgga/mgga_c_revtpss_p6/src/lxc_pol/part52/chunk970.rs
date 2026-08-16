//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 970/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk970(t1579: f64, t7398: f64, t7071: f64, t72: f64, t8006: f64, t686: f64, t25375: f64, t25387: f64, t27240: f64, t25246: f64, t25257: f64, t25267: f64, t26450: f64, t26454: f64, t27222: f64, t27224: f64, t27226: f64, t27228: f64, t27230: f64, t27232: f64, t27234: f64, t27236: f64) -> (f64, f64, f64, f64) {
    let t28309 = t7398 * t1579;
    let t28310 = t7071 * t28309;
    let t28313 = t8006 * t72;
    let t28314 = t28313 * t686;
    let t28315 = t25375 * t28314;
    let t28317 = t25387 * t28314;
    let t28330 = 0.11433071498151929859e-3_f64 * t27240;
    let t28331 = -0.50820002809285328225e-4_f64 * t25246 + 0.40015750243531754507e-2_f64 * t25267 + t27222 / 8.0_f64 + 0.17149607247227894789e-1_f64 * t27224 - 0.85748036236139473944e-3_f64 * t27226 - 0.50820002809285328225e-4_f64 * t27228 + 0.40015750243531754507e-2_f64 * t27230 + 0.34299214494455789578e-2_f64 * t27232 - 0.85748036236139473944e-3_f64 * t27234 + 0.34299214494455789578e-2_f64 * t27236 + t26450 - t26454 + t25257 + t28330;
    (t28310, t28315, t28317, t28331)
}
