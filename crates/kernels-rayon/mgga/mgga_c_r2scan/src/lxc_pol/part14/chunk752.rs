//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 752/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk752(t538: f64, t6064: f64, t6155: f64, t2110: f64, t3436: f64, t22: f64, t6: f64, t506: f64, t2162: f64, t3303: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6156 = t538 * t6064;
    let t6157 = t6155 * t6156;
    let t6159 = t3436 * t2110;
    let t6161 = t22 * t6;
    let t6162 = t506 * t6161;
    let t6164 = 0.14457274399185490173e-4_f64 * t6159 * t2162 * t6162;
    let t6165 = t545 * t3303;
    (t6156, t6157, t6159, t6161, t6162, t6164, t6165)
}
