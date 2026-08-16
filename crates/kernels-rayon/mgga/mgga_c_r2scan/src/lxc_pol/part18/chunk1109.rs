//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1109/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1109(t39885: f64, t8243: f64, t2605: f64, t37699: f64, t10833: f64, t980: f64, t38145: f64, t6535: f64, t8089: f64, t261: f64, t3299: f64, t7386: f64) -> (f64, f64, f64, f64, f64) {
    let t40102 = t39885 * t8243;
    let t40103 = 0.19514881078765566037e-1_f64 * t40102;
    let t40107 = t37699 * t2605;
    let t40109 = t980 * t10833;
    let t40131 = t6535 * t38145 * t8089;
    let t40137 = t3299 * t261 * t7386;
    (t40103, t40107, t40109, t40131, t40137)
}
