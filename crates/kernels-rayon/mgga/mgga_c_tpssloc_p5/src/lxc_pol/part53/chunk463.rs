//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 463/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk463(t1307: f64, t550: f64, t1291: f64, t2663: f64, t1284: f64, t67: f64, t758: f64, t2225: f64, t522: f64, t2221: f64, t2516: f64, t521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3807 = t550 * t1307;
    let t3813 = 0.24415263074675393405e-3_f64 * t1291 * t2663;
    let t3814 = t1284 * t67;
    let t3815 = t3814 * t758;
    let t3819 = 20.0_f64 * t2225 * t522;
    let t3821 = 12.0_f64 * t2221 * t522;
    let t3824 = t521 * t2516;
    (t3807, t3813, t3815, t3819, t3821, t3824)
}
