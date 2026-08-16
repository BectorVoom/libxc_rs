//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1156/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1156(t10833: f64, t980: f64, t25746: f64, t3332: f64, t7628: f64, t27177: f64, t6165: f64, t24786: f64, t24790: f64, t7614: f64, t10760: f64, t25303: f64, t6085: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40109 = t980 * t10833;
    let t40114 = t7628 * t3332 * t25746;
    let t40117 = t6165 * t3332 * t27177;
    let t40120 = t6165 * t3332 * t24786;
    let t40123 = t7614 * t3332 * t24790;
    let t40128 = t6085 * t10760 * t25303;
    (t40109, t40114, t40117, t40120, t40123, t40128)
}
