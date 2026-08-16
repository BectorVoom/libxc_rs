//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1043/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1043(t7780: f64, t7784: f64, t27055: f64, t7788: f64, t1281: f64, t7807: f64, t2201: f64, t3668: f64, t8027: f64, t911: f64, t2167: f64, t4527: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27080 = t7780 * t7784;
    let t27087 = t7788 * t27055;
    let t27100 = t7807 * t1281;
    let t27141 = t2201 * t3668;
    let t27731 = t911 * t8027;
    let t27733 = t4527 * t2167;
    (t27080, t27087, t27100, t27141, t27731, t27733)
}
