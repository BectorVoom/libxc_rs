//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 873/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk873(t1385: f64, t31558: f64, t22635: f64, t1992: f64, t794: f64, t8611: f64, t6897: f64, t225: f64, t567: f64, t7191: f64, t214: f64, t1985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31559 = t31558 * t1385;
    let t31560 = t22635 * t31559;
    let t31561 = t1992 * t31560;
    let t31569 = t794 * t8611;
    let t31570 = t6897 * t31569;
    let t31589 = t7191 * t225 * t567;
    let t31590 = t214 * t31589;
    let t31591 = t1985 * t31590;
    (t31559, t31560, t31561, t31569, t31570, t31589, t31590, t31591)
}
