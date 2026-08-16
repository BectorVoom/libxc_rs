//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 677/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk677(t2225: f64, t522: f64, t2221: f64, t2223: f64, t2516: f64, t521: f64, t17: f64, t1284: f64, t750: f64, t1285: f64, t592: f64, t1287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3819 = 20.0_f64 * t2225 * t522;
    let t3821 = 12.0_f64 * t2221 * t522;
    let t3823 = 32.0_f64 * t2223 * t522;
    let t3824 = t521 * t2516;
    let t3825 = t17 * t3824;
    let t3826 = t1284 * t750;
    let t3827 = t17 * t3826;
    let t3828 = 2.0_f64 * t3827;
    let t3829 = t592 * t1285;
    let t3830 = 8.0_f64 * t3829;
    let t3832 = 8.0_f64 * t592 * t1287;
    (t3819, t3821, t3823, t3824, t3825, t3826, t3828, t3830, t3832)
}
