//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 922/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk922(t131: f64, t8511: f64, t9239: f64, t113875: f64, t1862: f64, t31680: f64, t22573: f64, t8606: f64, t111: f64, t8646: f64, t7537: f64, t857: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115894 = t8511 * t131;
    let t115895 = t9239 * t115894;
    let t115903 = t113875 * t1862;
    let t115907 = t9239 * t31680;
    let t115925 = t8606 * t22573;
    let t115984 = t8646 * t111;
    let t118472 = t857 * t7537;
    (t115895, t115903, t115907, t115925, t115984, t118472)
}
