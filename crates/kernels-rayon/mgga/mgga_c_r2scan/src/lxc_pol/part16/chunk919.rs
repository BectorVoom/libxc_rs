//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 919/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk919(t423: f64, t58: f64, t597: f64, t10649: f64, t10648: f64, t2281: f64, t3428: f64, t3430: f64, t3308: f64, t3457: f64, t3429: f64, t4176: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10650 = t58 * t423;
    let t10651 = t10650 * t597;
    let t10652 = t10649 * t10651;
    let t10653 = t10648 * t10652;
    let t10655 = t2281 * t3428;
    let t10656 = t10655 * t3430;
    let t10657 = 0.15243824895787514157e-3_f64 * t10656;
    let t10659 = t3308 * t3457;
    let t10660 = t3429 * t10659;
    let t10662 = t4176 * t795;
    (t10650, t10651, t10652, t10653, t10655, t10657, t10659, t10660, t10662)
}
