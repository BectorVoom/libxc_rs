//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1298/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1298(t31687: f64, t9239: f64, t31677: f64, t131: f64, t2240: f64, t23966: f64, t31684: f64, t31680: f64, t9231: f64, t8511: f64, t113875: f64, t1862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t115876 = t9239 * t31687;
    let t115877 = t115876 * t31677;
    let t115888 = t2240 * t23966 * t131;
    let t115889 = t115888 * t31684;
    let t115891 = t9231 * t31680;
    let t115894 = t8511 * t131;
    let t115895 = t9239 * t115894;
    let t115903 = t113875 * t1862;
    (t115876, t115877, t115888, t115889, t115891, t115894, t115895, t115903)
}
