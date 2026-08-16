//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 990/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk990(t25224: f64, t6555: f64, t6552: f64, t1911: f64, t4300: f64, t2718: f64, t1519: f64, t828: f64, t232: f64, t6646: f64, t1888: f64, t13384: f64) -> (f64, f64, f64, f64, f64) {
    let t25229 = t25224 * t6555;
    let t25230 = t6552 * t25229;
    let t25232 = t1911 * t4300;
    let t25233 = t2718 * t25232;
    let t25236 = t1519 * t828;
    let t25237 = t25236 * t232;
    let t25238 = t6646 * t25237;
    let t25239 = t1888 * t25238;
    let t25241 = t13384 * t232;
    (t25230, t25232, t25233, t25239, t25241)
}
