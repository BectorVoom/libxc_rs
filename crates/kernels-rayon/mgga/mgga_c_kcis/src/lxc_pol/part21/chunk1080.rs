//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1080/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1080(t1121: f64, t1130: f64, t1133: f64, t26760: f64, t1092: f64, t2635: f64, t7704: f64, t4947: f64, t3225: f64, t342: f64, t3229: f64, t303: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26761 = t1130 * t1121;
    let t26762 = t26761 * t1133;
    let t26763 = t26760 * t26762;
    let t26764 = t1092 * t26763;
    let t26766 = t7704 * t2635;
    let t26767 = t4947 * t26766;
    let t26772 = t342 * t3225;
    let t26773 = t26772 * t3229;
    let t26774 = t303 * t26773;
    (t26762, t26763, t26764, t26766, t26767, t26773, t26774)
}
