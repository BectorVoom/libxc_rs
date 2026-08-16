//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1053/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1053(t2034: f64, t33228: f64, t1990: f64, t46833: f64, t10043: f64, t1979: f64, t1982: f64, t458: f64, t1971: f64, t236: f64, t38454: f64, t6096: f64) -> (f64, f64, f64, f64) {
    let t47295 = t33228 * t2034;
    let t47302 = t46833 * t1990;
    let t47306 = t10043 * t458 * t1979 * t1982;
    let t47310 = t38454 * t1971 * t236 * t6096;
    (t47295, t47302, t47306, t47310)
}
