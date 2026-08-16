//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1130/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1130(t40228: f64, t40251: f64, t40779: f64, t40788: f64, t40808: f64, t12382: f64, t12386: f64, t12388: f64, t12394: f64, t12581: f64, t12583: f64, t12584: f64, t39149: f64, t39150: f64, t39151: f64, t39152: f64, t39153: f64, t39154: f64, t39155: f64, t39156: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41753 = 0.32524801797942610062e-3_f64 * t40228;
    let t41770 = 0.35707476898255463229e0_f64 * t40251;
    let t41858 = 22.0_f64 / 9.0_f64 * t40779;
    let t41864 = 44.0_f64 / 9.0_f64 * t40788;
    let t41872 = 22.0_f64 / 9.0_f64 * t40808;
    let t42376 = t12382 - t39149 + t12386 - t39150 + t39151 - t39152 - t12388 + t12581 + t39153 - t39154 + t12583 + t12584 - t12394 - t39155 + t39156;
    (t41753, t41770, t41858, t41864, t41872, t42376)
}
