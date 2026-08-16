//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 970/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk970(t1137: f64, t7640: f64, t5693: f64, t1843: f64, t287: f64, t2105: f64, t2029: f64, t2916: f64, t2923: f64, t302: f64, t2009: f64, t2900: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7641 = t1137 * t7640;
    let t7642 = t5693 * t7641;
    let t7648 = t287 * t1843;
    let t7649 = t1137 * t7648;
    let t7650 = t2105 * t7649;
    let t7653 = t2916 * t2029;
    let t7654 = t7653 * t2923;
    let t7655 = t302 * t7654;
    let t7658 = t2009 * t287;
    let t7659 = t2900 * t7658;
    (t7641, t7642, t7648, t7649, t7650, t7653, t7654, t7655, t7658, t7659)
}
