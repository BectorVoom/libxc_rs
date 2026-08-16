//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 645/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk645(t1979: f64, t3625: f64, t730: f64, t154: f64, t2089: f64, t3542: f64, t3515: f64, t742: f64, t1123: f64) -> (f64, f64, f64, f64, f64) {
    let t3626 = t3625 * t1979;
    let t3628 = 0.17315859105681463759e2_f64 * t730 * t3626;
    let t3631 = t154 * t2089 * t3542;
    let t3635 = t154 * t742 * t3515;
    let t3638 = t1123 * t1123;
    (t3626, t3628, t3631, t3635, t3638)
}
