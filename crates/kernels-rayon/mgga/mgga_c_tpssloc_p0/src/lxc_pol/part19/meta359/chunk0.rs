//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1304/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1304(t41666: f64, t42308: f64, t10321: f64, t1041: f64, t248: f64, t3051: f64, t10459: f64, t3117: f64, t10469: f64, t990: f64, t10471: f64, t10875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42309 = t42308 * t41666;
    let t42322 = t1041 * t248 * t3051 * t10321;
    let t42324 = t3117 * t10459;
    let t42332 = t990 * t10469;
    let t42333 = t42332 * t10471;
    let t42334 = t42333 * t10875;
    (t42309, t42322, t42324, t42332, t42333, t42334)
}
