//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 666/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk666(t11627: f64, t723: f64, t1445: f64, t2925: f64, t2949: f64, t11628: f64, t11632: f64, t1457: f64, t11623: f64, t11724: f64, t701: f64, t2089: f64, t3614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11868 = t11627 * t723;
    let t11869 = t1445 * t11868;
    let t11874 = t2949 * t2925;
    let t11875 = t1445 * t11874;
    let t11878 = t1445 * t11628;
    let t11881 = t1445 * t11632;
    let t11884 = t1457 * t11632;
    let t11887 = t1457 * t11623;
    let t11890 = t11724 * t701;
    let t11891 = t1445 * t11890;
    let t11894 = t2089 * t3614;
    (t11869, t11875, t11878, t11881, t11884, t11887, t11891, t11894)
}
