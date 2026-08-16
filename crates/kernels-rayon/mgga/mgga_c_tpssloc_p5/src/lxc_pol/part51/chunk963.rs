//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 963/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk963(t23634: f64, t6743: f64, t23384: f64, t6790: f64, t6733: f64, t6796: f64, t995: f64, t6802: f64, t614: f64, t6794: f64, t131: f64, t350: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23635 = t6743 * t23634;
    let t23642 = t23384 * t6790;
    let t23657 = t6733 * t6743;
    let t23665 = t6796 * t995;
    let t23666 = t23665 * t6802;
    let t23668 = t614 * t6794;
    let t23669 = t23668 * t131;
    let t23670 = t23669 * t350;
    (t23635, t23642, t23657, t23665, t23666, t23670)
}
