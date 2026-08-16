//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1097/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1097(t11511: f64, t11513: f64, t11516: f64, t11520: f64, t11524: f64, t11526: f64, t10637: f64, t10639: f64, t10640: f64, t10665: f64, t10671: f64, t10690: f64, t10917: f64, t11029: f64, t11167: f64, t11169: f64) -> f64 {
    let t39159 = 3.0_f64 * t11511;
    let t39160 = 2.0_f64 * t11513;
    let t39161 = t11516 / 2.0_f64;
    let t39162 = 15.0_f64 / 8.0_f64 * t11520;
    let t39163 = t11524 / 2.0_f64;
    let t39164 = t11526 / 2.0_f64;
    let t39165 = t39159 + t39160 - t10637 + t10639 + t10640 + t11029 + t39161 - t39162 - t10665 + t10671 + t11167 + t11169 - t10690 + t10917 + t39163 + t39164;
    t39165
}
