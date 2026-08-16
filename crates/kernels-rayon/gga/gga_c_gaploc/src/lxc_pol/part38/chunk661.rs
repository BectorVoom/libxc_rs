//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 661/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk661(t3630: f64, t590: f64, t3601: f64, t5241: f64, t1890: f64, t3614: f64, t11604: f64, t1445: f64, t11622: f64, t723: f64, t11609: f64, t11595: f64, t313: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11777 = t3630 * t590;
    let t11780 = t5241 * t3601;
    let t11781 = t11780 * t590;
    let t11784 = t1890 * t3614;
    let t11785 = t11784 * t590;
    let t11788 = t1445 * t11604;
    let t11791 = t11622 * t723;
    let t11792 = t1445 * t11791;
    let t11795 = t1445 * t11609;
    let t11798 = t313 * t11595;
    (t11777, t11780, t11781, t11784, t11785, t11788, t11792, t11795, t11798)
}
