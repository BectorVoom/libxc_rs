//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1126/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1126(t1049: f64, t225: f64, t344: f64, t10189: f64, t1926: f64, t221: f64, t1921: f64, t6733: f64, t23383: f64, t6712: f64, t697: f64, t111: f64, t7002: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82417 = t344 * t1049 * t225;
    let t82431 = t1926 * t221 * t10189;
    let t82502 = t6733 * t1921;
    let t82573 = t6712 * t23383;
    let t82631 = t221 * t697;
    let t82632 = t1926 * t82631;
    let t83980 = t7002 * t111;
    (t82417, t82431, t82502, t82573, t82632, t83980)
}
