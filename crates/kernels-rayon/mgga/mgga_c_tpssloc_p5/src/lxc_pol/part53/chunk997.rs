//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 997/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk997(t3886: f64, t7936: f64, t1385: f64, t1992: f64, t22635: f64, t31559: f64, t90566: f64, t33246: f64, t6883: f64, t1985: f64, t214: f64, t225: f64, t27051: f64, t567: f64) -> (f64, f64, f64, f64) {
    let t122142 = t3886 * t7936;
    let t122145 = t1992 * t22635 * t122142 * t1385;
    let t122150 = t1992 * t90566 * t31559;
    let t122152 = t6883 * t33246;
    let t122160 = t1985 * t214 * t27051 * t225 * t567;
    (t122145, t122150, t122152, t122160)
}
