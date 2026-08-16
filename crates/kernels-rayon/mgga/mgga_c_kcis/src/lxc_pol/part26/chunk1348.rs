//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1348/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1348(t1552: f64, t20961: f64, t22212: f64, t6028: f64, t7948: f64, t8191: f64, t97727: f64, t22699: f64, t491: f64, t7949: f64, t1928: f64, t6015: f64) -> (f64, f64, f64, f64, f64) {
    let t103035 = t20961 * t1552;
    let t103038 = t7948 * t6028 * t22212;
    let t103040 = t97727 * t8191;
    let t103042 = t22699 * t491;
    let t103043 = t103042 * t7949;
    let t103045 = t6015 * t1928;
    (t103035, t103038, t103040, t103043, t103045)
}
