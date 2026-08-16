//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 965/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk965(t1036: f64, t6759: f64, t3: f64, t6740: f64, t23476: f64, t343: f64, t23384: f64, t6692: f64, t1049: f64, t6688: f64, t1054: f64, t1065: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23560 = t6759 * t1036;
    let t23562 = t6740 * t3;
    let t23563 = t23476 * t343;
    let t23564 = t23562 * t23563;
    let t23579 = t23384 * t6692;
    let t23581 = t6688 * t1049;
    let t23587 = t1054 * t1065;
    (t23560, t23562, t23564, t23579, t23581, t23587)
}
