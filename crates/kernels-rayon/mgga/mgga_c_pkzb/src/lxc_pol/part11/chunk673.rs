//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 673/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk673(t2320: f64, t3840: f64, t898: f64, t154: f64, t3730: f64, t907: f64, t275: f64, t3324: f64) -> (f64, f64, f64, f64) {
    let t3841 = t3840 * t2320;
    let t3843 = 0.17315859105681463759e2_f64 * t898 * t3841;
    let t3846 = t154 * t907 * t3730;
    let t3849 = t3324 * t275;
    (t3841, t3843, t3846, t3849)
}
