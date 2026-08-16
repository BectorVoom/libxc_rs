//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1263/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1263(t1862: f64, t8308: f64, t113875: f64, t31867: f64, t9239: f64, t31863: f64, t9231: f64, t131: f64, t8662: f64, t2240: f64, t24525: f64, t39054: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t115833 = t8308 * t1862;
    let t115903 = t113875 * t1862;
    let t116082 = t9239 * t31867;
    let t116106 = t9239 * t31863;
    let t116111 = t9231 * t31863;
    let t116114 = t8662 * t131;
    let t116115 = t9239 * t116114;
    let t116119 = t2240 * t24525 * t131;
    let t116124 = t39054 * t8662;
    (t115833, t115903, t116082, t116106, t116111, t116114, t116115, t116119, t116124)
}
