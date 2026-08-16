//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1145/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1145(t25: f64, t6534: f64, t285: f64, t2909: f64, t6539: f64, t1003: f64, t417: f64, t18443: f64, t994: f64, t993: f64, t6533: f64, t9874: f64) -> (f64, f64, f64, f64) {
    let t19255 = t25 * t6534;
    let t19256 = t285 * t19255;
    let t19258 = t2909 * t6539;
    let t19259 = t19258 * t1003;
    let t19260 = t417 * t19259;
    let t19263 = t994 * t18443;
    let t19264 = t993 * t19263;
    let t19267 = t9874 * t6533;
    (t19256, t19260, t19264, t19267)
}
