//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1455/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1455(t2240: f64, t27363: f64, t8301: f64, t12571: f64, t31863: f64, t116114: f64, t39063: f64, t45844: f64, t8662: f64, t33676: f64, t9239: f64, t191: f64, t192: f64, t27903: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122964 = t2240 * t8301 * t27363;
    let t122976 = t12571 * t31863;
    let t122979 = t39063 * t116114;
    let t122988 = t45844 * t8662;
    let t123001 = t9239 * t33676;
    let t123111 = t27903 * t191 * t192;
    (t122964, t122976, t122979, t122988, t123001, t123111)
}
