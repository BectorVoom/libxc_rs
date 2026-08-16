//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1179/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1179(t11888: f64, t6654: f64, t1276: f64, t2391: f64, t3366: f64, t1070: f64, t8395: f64, t11047: f64, t23498: f64, t11050: f64, t8358: f64, t11885: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40794 = t6654 * t11888;
    let t40797 = t1276 * t3366 * t2391;
    let t40800 = t1276 * t1070 * t8395;
    let t40802 = t23498 * t11047;
    let t40804 = t8358 * t11050;
    let t40806 = t6654 * t11885;
    (t40794, t40797, t40800, t40802, t40804, t40806)
}
