//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1252/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1252(t39244: f64, t39251: f64, t39255: f64, t39247: f64, t41160: f64, t41162: f64, t41165: f64, t41168: f64, t41170: f64, t41173: f64, t41176: f64, t41179: f64, t41182: f64, t41185: f64, t41188: f64) -> f64 {
    let t42162 = 0.1440846329149835838e-2_f64 * t39244;
    let t42164 = 0.1440846329149835838e-2_f64 * t39251;
    let t42165 = 0.1440846329149835838e-2_f64 * t39255;
    let t42166 = -t41160 - t41162 - t41165 + t41168 + t41170 + t41173 + t42162 - 0.72042316457491791901e-3_f64 * t39247 + t42164 + t42165 + t41176 + t41179 + t41182 + t41185 - t41188;
    t42166
}
