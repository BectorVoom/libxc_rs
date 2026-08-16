//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 520/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk520(t1343: f64, t3856: f64, t820: f64, t248: f64, t2691: f64, t557: f64, t555: f64, t1361: f64, t835: f64, t1336: f64, t1369: f64, t1995: f64, t241: f64) -> (f64, f64, f64, f64, f64) {
    let t3858 = t1343 * t820 * t3856;
    let t3862 = t2691 * t557 * t248;
    let t3864 = 119.0_f64 / 13824.0_f64 * t555 * t3862;
    let t3865 = t1361 * t835;
    let t3866 = t1336 * t3865;
    let t3867 = t3866 * t1369;
    let t3869 = t241 * t1995;
    (t3858, t3862, t3864, t3867, t3869)
}
