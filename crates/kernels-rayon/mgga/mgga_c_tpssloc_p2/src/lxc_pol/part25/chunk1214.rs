//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1214/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1214(t225: f64, t24200: f64, t10049: f64, t24297: f64, t24314: f64, t24325: f64, t2597: f64, t2713: f64, t2718: f64, t2720: f64, t2742: f64, t7092: f64, t7106: f64, t82129: f64, t82131: f64, t82135: f64, t82138: f64, t855: f64, t866: f64) -> f64 {
    let t85079 = t24200 * t225;
    let t85093 = -18.0_f64 * t2713 * t24314 + 6.0_f64 * t10049 * t7092 + 0.9869604401089358619e-1_f64 * t82129 + 12.0_f64 * t2713 * t24325 - 3.0_f64 * t85079 * t866 + 6.0_f64 * t24297 * t2720 + 6.0_f64 * t855 * t2718 * t7106 * t2742 - 0.11514538467937585055e0_f64 * t82131 + 0.49348022005446793095e-1_f64 * t82135 - 0.9869604401089358619e-1_f64 * t82138 - 18.0_f64 * t2597 * t24314;
    t85093
}
