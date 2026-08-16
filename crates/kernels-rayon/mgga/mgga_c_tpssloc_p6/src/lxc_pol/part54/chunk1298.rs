//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1298/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1298(t31863: f64, t9231: f64, t131: f64, t8662: f64, t9239: f64, t2240: f64, t24525: f64, t39054: f64, t22573: f64, t8689: f64, t2098: f64, t7426: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t116111 = t9231 * t31863;
    let t116114 = t8662 * t131;
    let t116115 = t9239 * t116114;
    let t116119 = t2240 * t24525 * t131;
    let t116124 = t39054 * t8662;
    let t116135 = t8689 * t22573;
    let t117407 = t2098 * t7426;
    (t116111, t116114, t116115, t116119, t116124, t116135, t117407)
}
