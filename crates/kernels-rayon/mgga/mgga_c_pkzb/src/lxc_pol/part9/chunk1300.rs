//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1300/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1300(t2234: f64, t2240: f64, t8198: f64, t3069: f64, t6201: f64, t2198: f64, t6199: f64, t3073: f64, t6193: f64, t1184: f64, t18589: f64, t18592: f64, t6143: f64) -> (f64, f64, f64, f64) {
    let t22840 = 0.48245938496077605201e2_f64 * t2240 * t8198 * t2234;
    let t22841 = t3069 * t6201;
    let t22844 = 0.1551780387578202009e4_f64 * t6199 * t22841 * t2198;
    let t22847 = 0.16081979498692535067e2_f64 * t2240 * t3073 * t6193;
    let t22851 = 0.24955700379505800916e5_f64 * t18589 * t1184 * t18592 * t6143;
    (t22840, t22844, t22847, t22851)
}
