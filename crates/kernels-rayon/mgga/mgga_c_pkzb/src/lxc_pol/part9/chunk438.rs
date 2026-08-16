//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 438/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk438(t158: f64, t1739: f64, t1740: f64, t133: f64, t614: f64, t1634: f64, t1692: f64, t596: f64, t160: f64, t162: f64, t594: f64, t597: f64) -> (f64, f64, f64, f64) {
    let t1742 = (t1739 + t1740) * t158;
    let t1746 = t133 * t614;
    let t1747 = t1746 * t1634;
    let t1750 = t596 * t1692;
    let t1753 = -12.0_f64 * t160 * t1747 + 3.0_f64 * t160 * t1750 - t162 * t1742 + 6.0_f64 * t594 * t597;
    (t1742, t1747, t1750, t1753)
}
