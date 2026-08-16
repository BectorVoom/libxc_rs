//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 579/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk579(t1107: f64, t2712: f64, t2711: f64, t2785: f64, t450: f64, t475: f64, t1183: f64, t177: f64, t737: f64, t1193: f64, t2206: f64, t198: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3137 = t2712 * t1107;
    let t3138 = t2711 * t3137;
    let t3139 = t2785 * t450;
    let t3153 = t475 * t475;
    let t3154 = 1.0_f64 / t3153;
    let t3178 = t1183 * t177;
    let t3179 = t3178 * t737;
    let t3182 = 0.5848223622634646207e0_f64 * t1193 * t2206;
    let t3183 = t198 * t508;
    (t3138, t3139, t3153, t3154, t3178, t3179, t3182, t3183)
}
