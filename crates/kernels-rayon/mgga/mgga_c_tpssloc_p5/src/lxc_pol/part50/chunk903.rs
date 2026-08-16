//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 903/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk903(t1011: f64, t3131: f64, t1049: f64, t362: f64, t23384: f64, t6787: f64, t3216: f64, t6818: f64, t11094: f64, t1958: f64, t2752: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23678 = t1011 * t3131;
    let t23685 = t362 * t1049;
    let t23712 = t23384 * t6787;
    let t23738 = t6818 * t3216;
    let t23742 = t1958 * t11094;
    let t23788 = t2752 * t28;
    (t23678, t23685, t23712, t23738, t23742, t23788)
}
