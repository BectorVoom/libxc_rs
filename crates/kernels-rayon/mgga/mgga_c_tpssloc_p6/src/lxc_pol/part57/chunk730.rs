//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 730/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk730(t23472: f64, t25641: f64, t25637: f64, t343: f64, t23562: f64, t344: f64, t7573: f64, t6740: f64, t23384: f64, t7566: f64, t1054: f64, t1634: f64) -> (f64, f64, f64, f64, f64) {
    let t25642 = t23472 * t25641;
    let t25644 = t25637 * t343;
    let t25645 = t23562 * t25644;
    let t25682 = t7573 * t344;
    let t25683 = t6740 * t25682;
    let t25736 = t23384 * t7566;
    let t25749 = t1054 * t1634;
    (t25642, t25645, t25683, t25736, t25749)
}
