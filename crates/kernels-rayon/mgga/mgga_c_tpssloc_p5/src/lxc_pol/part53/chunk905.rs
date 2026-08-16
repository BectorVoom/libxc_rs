//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 905/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk905(t26193: f64, t8621: f64, t1985: f64, t225: f64, t567: f64, t7918: f64, t214: f64, t1842: f64, t31558: f64, t22635: f64, t1992: f64, t1799: f64, t31549: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33240 = t26193 * t8621;
    let t33241 = t1985 * t33240;
    let t33245 = t7918 * t225 * t567;
    let t33246 = t214 * t33245;
    let t33247 = t1985 * t33246;
    let t33249 = t31558 * t1842;
    let t33250 = t22635 * t33249;
    let t33251 = t1992 * t33250;
    let t33272 = t31549 * t1799;
    (t33240, t33241, t33245, t33246, t33247, t33249, t33250, t33251, t33272)
}
