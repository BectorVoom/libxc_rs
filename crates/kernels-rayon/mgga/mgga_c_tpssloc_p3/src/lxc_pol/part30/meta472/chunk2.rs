//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1766/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1766(t23665: f64, t6802: f64, t614: f64, t6794: f64, t131: f64, t350: f64, t23602: f64, t3127: f64, t1011: f64, t3131: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23666 = t23665 * t6802;
    let t23668 = t614 * t6794;
    let t23669 = t23668 * t131;
    let t23670 = t23669 * t350;
    let t23677 = t23602 * t3127;
    let t23678 = t1011 * t3131;
    (t23666, t23668, t23669, t23670, t23677, t23678)
}
