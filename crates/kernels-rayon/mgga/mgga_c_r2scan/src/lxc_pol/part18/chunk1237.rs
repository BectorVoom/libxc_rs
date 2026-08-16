//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1237/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1237(t11496: f64, t2850: f64, t3262: f64, t3263: f64, t1108: f64, t2881: f64, t3685: f64, t42976: f64, t43716: f64, t43720: f64, t43724: f64, t43728: f64, t43732: f64, t43735: f64, t43739: f64, t43742: f64, t43747: f64, t43750: f64, t43752: f64, t43754: f64, t43756: f64, t9782: f64) -> (f64, f64) {
    let t43757 = t11496 * t2850;
    let t43760 = 3.0_f64 / 2.0_f64 * t3262 * t3263 * t43757;
    let t43761 = t1108 * t9782 + 2.0_f64 * t2881 * t3685 + t42976 + t43716 - t43720 + t43724 + t43728 - t43732 - t43735 - t43739 - t43742 - t43747 - t43750 - t43752 + t43754 + t43756 - t43760;
    (t43760, t43761)
}
