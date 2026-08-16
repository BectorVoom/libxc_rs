//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 965/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk965(t343: f64, t381: f64, t6690: f64, t25712: f64, t4347: f64, t6689: f64, t7561: f64, t968: f64, t1920: f64, t1625: f64, t6688: f64, t6691: f64) -> (f64, f64, f64, f64, f64) {
    let t25796 = t343 * t381;
    let t25797 = t25796 * t6690;
    let t25798 = t25712 * t25797;
    let t25801 = t6690 * t4347;
    let t25802 = t6689 * t25801;
    let t25806 = t968 * t7561;
    let t25807 = t1920 * t25806;
    let t25810 = t6688 * t1625;
    let t25811 = t25810 * t6691;
    (t25798, t25801, t25802, t25807, t25811)
}
