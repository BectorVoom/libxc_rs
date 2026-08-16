//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1250/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1250(t26717: f64, t26739: f64, t1250: f64, t33827: f64, t15573: f64, t2173: f64, t26792: f64, t26857: f64, t7699: f64, t26742: f64, t46978: f64, t7710: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93714 = t26739 * t26717;
    let t93718 = t33827 * t1250;
    let t93728 = t2173 * t15573 * t26792;
    let t93742 = t26857 * t7699;
    let t93750 = t26742 * t7699;
    let t93759 = t2173 * t46978 * t7710;
    (t93714, t93718, t93728, t93742, t93750, t93759)
}
