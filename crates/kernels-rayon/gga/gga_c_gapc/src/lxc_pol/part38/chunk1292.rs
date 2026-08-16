//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1292/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1292(t10110: f64, t11691: f64, t2493: f64, t3243: f64, t640: f64, t10336: f64, t11695: f64, t3209: f64, t35846: f64, t923: f64, t22949: f64, t22954: f64, t268: f64, t35424: f64, t6148: f64, t7875: f64) -> (f64, f64, f64, f64, f64) {
    let t35940 = t10110 * t11691;
    let t35943 = t3243 * t640 * t2493;
    let t35945 = t10336 * t11695;
    let t35948 = t3209 * t35846 * t923;
    let t35954 = t35424 * t268 * t22949 * t6148 * t7875 * t22954;
    (t35940, t35943, t35945, t35948, t35954)
}
