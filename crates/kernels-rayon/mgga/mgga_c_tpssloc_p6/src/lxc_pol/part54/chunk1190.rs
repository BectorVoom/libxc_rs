//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1190/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1190(t7254: f64, t8301: f64, t2240: f64, t3701: f64, t7216: f64, t2039: f64, t7408: f64, t645: f64, t8513: f64, t8824: f64, t31: f64, t63: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31867 = t8301 * t7254;
    let t31868 = t2240 * t31867;
    let t32193 = t3701 * t7216;
    let t32318 = t7408 * t2039;
    let t32328 = t8513 * t8824 * t645;
    let t32331 = t63 * t31;
    (t31867, t31868, t32193, t32318, t32328, t32331)
}
