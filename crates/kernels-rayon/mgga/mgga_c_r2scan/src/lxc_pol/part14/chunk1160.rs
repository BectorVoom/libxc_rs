//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1160/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1160(t10740: f64, t980: f64, t10760: f64, t24750: f64, t6085: f64, t24070: f64, t6093: f64, t29418: f64, t3293: f64, t132: f64, t537: f64, t7322: f64) -> (f64, f64, f64, f64, f64) {
    let t40185 = t980 * t10740;
    let t40188 = t6085 * t10760 * t24750;
    let t40191 = t6093 * t10760 * t24070;
    let t40194 = t3293 * t29418;
    let t40195 = t132 * t537;
    let t40197 = t40194 * t40195 * t7322;
    (t40185, t40188, t40191, t40195, t40197)
}
