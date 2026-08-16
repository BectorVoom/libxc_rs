//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 733/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk733(t23110: f64, t7524: f64, t23185: f64, t1484: f64, t252: f64, t7510: f64, t814: f64, t7528: f64, t794: f64, t6562: f64, t1509: f64, t1902: f64) -> (f64, f64, f64, f64, f64) {
    let t25245 = t23110 * t7524;
    let t25246 = t23185 * t25245;
    let t25249 = t252 * t1484;
    let t25255 = t814 * t7510;
    let t25258 = t794 * t7528;
    let t25259 = t6562 * t25258;
    let t25261 = t1902 * t1509;
    (t25246, t25249, t25255, t25259, t25261)
}
