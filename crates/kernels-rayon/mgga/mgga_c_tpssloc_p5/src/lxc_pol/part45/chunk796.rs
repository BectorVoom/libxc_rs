//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 796/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk796(t1011: f64, t3131: f64, t3187: f64, t23677: f64, t1049: f64, t362: f64, t884: f64, t6784: f64, t2780: f64, t6785: f64, t225: f64, t23592: f64) -> (f64, f64, f64, f64) {
    let t23678 = t1011 * t3131;
    let t23679 = t3187 * t23678;
    let t23680 = t23677 * t23679;
    let t23685 = t362 * t1049;
    let t23686 = t23685 * t884;
    let t23687 = t6784 * t23686;
    let t23692 = t6785 * t2780;
    let t23693 = t6784 * t23692;
    let t23696 = t23592 * t225;
    (t23680, t23687, t23693, t23696)
}
