//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 682/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk682(t28859: f64, t875: f64, t7022: f64, t880: f64, t193: f64, t1253: f64, t824: f64, t6222: f64, t681: f64, t7023: f64, t28491: f64, t28494: f64, t28499: f64, t28504: f64, t28509: f64, t28514: f64, t28518: f64, t28522: f64, t28526: f64, t28529: f64, t28531: f64, t28536: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28860 = t28859 * t875;
    let t28862 = t7022 * t880;
    let t28863 = t193 * t28862;
    let t28868 = t1253 * t824;
    let t28869 = t6222 * t28868;
    let t28870 = t193 * t28869;
    let t28873 = t681 * t7023;
    let t28885 = t28491 / 3.0_f64 - t28494 / 12.0_f64 + t28499 + t28504 + t28509 + t28514 / 4.0_f64 - 2.0_f64 / 3.0_f64 * t28518 - 2.0_f64 / 3.0_f64 * t28522 + 2.0_f64 / 9.0_f64 * t28526 + t28529 / 6.0_f64 - t28531 / 9.0_f64 + t28536 / 3.0_f64;
    (t28860, t28863, t28868, t28870, t28873, t28885)
}
