//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1260/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1260(t40456: f64, t40460: f64, t37483: f64, t37488: f64, t37495: f64, t39083: f64, t40451: f64, t40485: f64, t41806: f64, t41808: f64, t41811: f64, t41814: f64, t41818: f64, t41821: f64, t41824: f64) -> f64 {
    let t42215 = 0.60975299583150056624e-3_f64 * t40456;
    let t42216 = 0.86737941314158990616e-4_f64 * t40460;
    let t42221 = -0.30487649791575028312e-3_f64 * t40451 - t42215 + t42216 + t41806 + t41808 + t39083 - t41811 + t41814 - 0.78064147182743091556e-3_f64 * t37483 + t41818 + 0.29810146462873361016e-2_f64 * t40485 + t41821 + 0.72042316457491791901e-3_f64 * t37488 + 0.1440846329149835838e-2_f64 * t37495 + t41824;
    t42221
}
