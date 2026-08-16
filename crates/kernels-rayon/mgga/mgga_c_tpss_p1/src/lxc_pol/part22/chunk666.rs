//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 666/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk666(t2785: f64, t3054: f64, t1113: f64, t1133: f64, t3073: f64, t466: f64, t1107: f64, t2712: f64, t2711: f64, t450: f64, t1141: f64, t1143: f64, t220: f64, t3110: f64, t3124: f64, t3125: f64, t468: f64) -> (f64, f64, f64, f64) {
    let t3126 = t2785 * t3054;
    let t3130 = t1133 * t1113;
    let t3134 = t466 * t3073;
    let t3137 = t2712 * t1107;
    let t3138 = t2711 * t3137;
    let t3139 = t2785 * t450;
    let t3144 = 2.0_f64 * t1141 * t1143 * t3130 + t1141 * t1143 * t3134 + t220 * t3110 * t468 + 2.0_f64 * t3124 * t3125 * t3126 - t3125 * t3138 * t3139;
    (t3126, t3138, t3139, t3144)
}
