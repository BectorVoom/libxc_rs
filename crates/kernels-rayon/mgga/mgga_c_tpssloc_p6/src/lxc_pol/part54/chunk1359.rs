//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1359/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1359(t33617: f64, t4034: f64, t652: f64, t7156: f64, t7467: f64, t6534: f64, t7890: f64, t1458: f64, t7039: f64, t1874: f64, t2035: f64, t4072: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120995 = 2.0_f64 * t4034 * t33617;
    let t120998 = 2.0_f64 * t652 * t7156 * t7467;
    let t121003 = 2.0_f64 * t652 * t7890 * t6534;
    let t121004 = t7039 * t1458;
    let t121006 = 2.0_f64 * t121004 * t1874;
    let t121007 = t2035 * t4072;
    (t120995, t120998, t121003, t121004, t121006, t121007)
}
