//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 893/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk893(t10125: f64, t10138: f64, t10147: f64, t10148: f64, t225: f64, t3023: f64, t1053: f64, t68: f64, t1065: f64, t3175: f64, t3021: f64, t3206: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10150 = t10125 + t10138 + t10147 + t10148;
    let t10160 = t3023 * t225;
    let t10163 = t1053 * t1053;
    let t10164 = 1.0_f64 / t10163;
    let t10165 = t68 * t10164;
    let t10166 = t3175 * t1065;
    let t10167 = t10165 * t10166;
    let t10170 = t3021 * t225;
    let t10181 = t1065 * t3206;
    (t10150, t10160, t10163, t10164, t10165, t10166, t10167, t10170, t10181)
}
