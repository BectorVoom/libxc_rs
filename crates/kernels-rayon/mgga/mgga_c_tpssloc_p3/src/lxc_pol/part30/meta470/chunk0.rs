//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1758/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1758(t23476: f64, t343: f64, t23562: f64, t23384: f64, t6692: f64, t1049: f64, t6688: f64, t1054: f64, t1065: f64, t1921: f64, t2978: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23563 = t23476 * t343;
    let t23564 = t23562 * t23563;
    let t23579 = t23384 * t6692;
    let t23581 = t6688 * t1049;
    let t23587 = t1054 * t1065;
    let t23588 = t1921 * t23587;
    let t23592 = t2978 * t344;
    (t23563, t23564, t23579, t23581, t23587, t23588, t23592)
}
