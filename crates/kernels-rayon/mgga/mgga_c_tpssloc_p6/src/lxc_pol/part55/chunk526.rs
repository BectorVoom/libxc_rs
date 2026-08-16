//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 526/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk526(t17: f64, t3824: f64, t1284: f64, t750: f64, t1285: f64, t592: f64, t1287: f64, t588: f64, t248: f64, t2691: f64, t557: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3825 = t17 * t3824;
    let t3826 = t1284 * t750;
    let t3827 = t17 * t3826;
    let t3829 = t592 * t1285;
    let t3832 = 8.0_f64 * t592 * t1287;
    let t3833 = t588 * t1285;
    let t3862 = t2691 * t557 * t248;
    let t3864 = 119.0_f64 / 13824.0_f64 * t555 * t3862;
    (t3825, t3827, t3829, t3832, t3833, t3862, t3864)
}
