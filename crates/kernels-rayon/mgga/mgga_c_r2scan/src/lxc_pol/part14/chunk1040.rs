//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1040/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1040(t494: f64, t9507: f64, t1553: f64, t24209: f64, t2531: f64, t6212: f64, t2252: f64, t2562: f64, t2185: f64, t2567: f64, t2599: f64, t3433: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25684 = t9507 * t494;
    let t25697 = t24209 * t1553;
    let t25737 = t6212 * t2531;
    let t25746 = t2562 * t2252;
    let t25813 = t2567 * t2185;
    let t25826 = t3433 * t2599;
    (t25684, t25697, t25737, t25746, t25813, t25826)
}
