//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2188/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2188(t3886: f64, t6439: f64, t1307: f64, t22633: f64, t22635: f64, t1985: f64, t26193: f64, t26202: f64, t6888: f64, t6891: f64, t97511: f64, t28116: f64, t80650: f64) -> (f64, f64, f64, f64) {
    let t97608 = t3886 * t6439;
    let t97611 = t22633 * t22635 * t97608 * t1307;
    let t97616 = t1985 * t26193 * t26202;
    let t97619 = t6888 * t97511 * t6891;
    let t97624 = t22633 * t80650 * t28116;
    (t97611, t97616, t97619, t97624)
}
