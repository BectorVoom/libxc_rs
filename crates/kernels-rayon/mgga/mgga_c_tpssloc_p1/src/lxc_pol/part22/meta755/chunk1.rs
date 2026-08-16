//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2538/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2538(t136: f64, t43761: f64, t71164: f64, t1100: f64, t71390: f64, t1113: f64, t71148: f64, t21794: f64, t699: f64, t11219: f64, t71158: f64, t71133: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71400 = t136 * t43761 * t71164;
    let t71403 = t1100 * t71390;
    let t71406 = t136 * t1113 * t71148;
    let t71408 = t699 * t21794;
    let t71411 = t136 * t11219 * t71158;
    let t71414 = t136 * t11219 * t71133;
    (t71400, t71403, t71406, t71408, t71411, t71414)
}
