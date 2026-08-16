//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1326/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1326(t118454: f64, t23788: f64, t2314: f64, t32677: f64, t4034: f64, t5107: f64, t652: f64, t8326: f64, t1845: f64, t6995: f64, t1799: f64, t1437: f64, t31: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119780 = t23788 * t118454;
    let t119824 = 2.0_f64 * t2314 * t32677;
    let t119826 = 2.0_f64 * t4034 * t32677;
    let t119830 = 2.0_f64 * t652 * t5107 * t8326;
    let t119832 = t1845 * t6995;
    let t119853 = t1799 * t6995;
    let t119878 = t1437 * t31;
    (t119780, t119824, t119826, t119830, t119832, t119853, t119878)
}
