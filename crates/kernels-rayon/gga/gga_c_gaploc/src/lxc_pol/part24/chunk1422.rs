//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1422/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1422(t35099: f64, t10241: f64, t20550: f64, t15482: f64, t20549: f64, t1: f64, t31740: f64, t544: f64, t10540: f64, t18067: f64, t2365: f64, t25730: f64, t4391: f64) -> (f64, f64, f64, f64, f64) {
    let t35100 = 0.2556195063594716645e0_f64 * t35099;
    let t35101 = t20550 * t10241;
    let t35104 = 0.34082600847929555269e0_f64 * t20549 * t15482 * t35101;
    let t35106 = t544 * t31740 * t1;
    let t35109 = t18067 * t10540;
    let t35110 = 0.59584149919750711116e-1_f64 * t35109;
    let t35112 = t4391 * t2365 * t25730;
    (t35100, t35104, t35106, t35110, t35112)
}
