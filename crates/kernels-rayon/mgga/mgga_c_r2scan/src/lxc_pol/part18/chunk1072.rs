//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1072/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1072(t38054: f64, t2116: f64, t57: f64, t6257: f64, t505: f64, t6159: f64, t6162: f64, t2096: f64, t2105: f64, t254: f64, t265: f64, t6079: f64) -> (f64, f64, f64, f64) {
    let t38055 = 0.19776387377308997907e1_f64 * t38054;
    let t38068 = t6257 * t57 * t2116;
    let t38069 = 0.98171973930797904389e-1_f64 * t38068;
    let t38130 = t6159 * t505 * t6162;
    let t38131 = 0.14457274399185490173e-4_f64 * t38130;
    let t38143 = t254 * t6079 * t2096 * t265 * t2105;
    (t38055, t38069, t38131, t38143)
}
