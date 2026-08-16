//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 809/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk809(t8679: f64, t8685: f64, t8690: f64, t8796: f64, t7702: f64, t7706: f64, t7712: f64, t7714: f64, t7719: f64, t7722: f64, t7724: f64, t7726: f64, t7728: f64, t8173: f64) -> (f64, f64, f64, f64) {
    let t38292 = 0.85129199786595678796e-5_f64 * t8679;
    let t38295 = 0.85129199786595678796e-5_f64 * t8685;
    let t38296 = 0.85129199786595678796e-5_f64 * t8690;
    let t38300 = 0.39914139006212695214e-1_f64 * t8796;
    let t38301 = -t38300 - t7702 - t7706 + t8173 - t7712 + t7714 - t7719 - t7722 + t7724 - t7726 - t7728;
    (t38292, t38295, t38296, t38301)
}
