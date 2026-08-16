//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 402/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk402(t1719: f64, t720: f64, t748: f64, t234: f64, t218: f64, t716: f64) -> (f64, f64, f64, f64) {
    let t1813 = t720 * t1719;
    let t1814 = t748 * t1813;
    let t1816 = 0.17315859105681463759e2_f64 * t234 * t1814;
    let t1818 = 1.0_f64 / t716 / t218;
    (t1813, t1814, t1816, t1818)
}
