//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 772/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk772(t1632: f64, t2526: f64, t551: f64, t566: f64, t2183: f64, t2666: f64, t2191: f64, t2667: f64, t2123: f64, t538: f64, t1568: f64, t1569: f64, t920: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7309 = t1632 * t2526;
    let t7310 = t551 * t7309;
    let t7312 = 0.69345773920434148506e0_f64 * t566 * t7310;
    let t7313 = t2183 * t2666;
    let t7317 = 0.23115257973478049502e0_f64 * t2667 * t2191;
    let t7321 = t2123 * t538;
    let t7337 = t2123 * t1568;
    let t7338 = t920 * t1569;
    (t7309, t7312, t7313, t7317, t7321, t7337, t7338)
}
