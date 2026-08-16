//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1240/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1240(t40779: f64, t40781: f64, t40788: f64, t40797: f64, t40804: f64, t40806: f64, t40808: f64, t37043: f64, t37048: f64, t37055: f64, t37069: f64, t37078: f64, t40790: f64, t40792: f64, t40794: f64, t40800: f64, t40802: f64, t40812: f64) -> f64 {
    let t41858 = 22.0_f64 / 9.0_f64 * t40779;
    let t41859 = 8.0_f64 / 3.0_f64 * t40781;
    let t41864 = 44.0_f64 / 9.0_f64 * t40788;
    let t41867 = 8.0_f64 / 3.0_f64 * t40797;
    let t41870 = 8.0_f64 / 3.0_f64 * t40804;
    let t41871 = 8.0_f64 / 3.0_f64 * t40806;
    let t41872 = 22.0_f64 / 9.0_f64 * t40808;
    let t41875 = -t41858 + t41859 - 8.0_f64 / 3.0_f64 * t37048 + 4.0_f64 * t37055 - 4.0_f64 / 3.0_f64 * t37069 + 88.0_f64 / 9.0_f64 * t37078 + t41864 + t40790 / 2.0_f64 + t40792 / 2.0_f64 + t40794 - t41867 + t40800 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t40802 - t41870 - t41871 + t41872 - 2.0_f64 / 3.0_f64 * t37043 - 3.0_f64 * t40812;
    t41875
}
