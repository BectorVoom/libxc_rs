//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 677/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk677(t3880: f64, t405: f64, t921: f64, t758: f64, t2371: f64, t3875: f64, t154: f64, t2352: f64, t3757: f64, t1167: f64, t394: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3881 = t405 * t3880;
    let t3882 = t3881 * t921;
    let t3883 = t758 * t3882;
    let t3886 = t3875 * t2371;
    let t3887 = t758 * t3886;
    let t3892 = t154 * t2352 * t3757;
    let t3898 = t394 * t1167;
    (t3881, t3882, t3883, t3886, t3887, t3892, t3898)
}
