//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1355/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1355(t115925: f64, t25971: f64, t24987: f64, t8644: f64, t101138: f64, t26161: f64, t31775: f64, t1441: f64, t6534: f64, t2040: f64, t33211: f64, t7050: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120899 = 3.0_f64 * t115925 * t25971;
    let t120900 = t24987 * t8644;
    let t120907 = 2.0_f64 * t26161 * t101138 * t31775;
    let t120908 = t1441 * t6534;
    let t120910 = 2.0_f64 * t120908 * t2040;
    let t120912 = 2.0_f64 * t33211 * t7050;
    (t120899, t120900, t120907, t120908, t120910, t120912)
}
