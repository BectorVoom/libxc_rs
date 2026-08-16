//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1352/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1352(t15482: f64, t20549: f64, t35101: f64, t10540: f64, t18067: f64, t2365: f64, t25730: f64, t4391: f64, t25580: f64, t20671: f64, t27007: f64, t31047: f64) -> (f64, f64, f64, f64, f64) {
    let t35104 = 0.34082600847929555269e0_f64 * t20549 * t15482 * t35101;
    let t35109 = t18067 * t10540;
    let t35110 = 0.59584149919750711116e-1_f64 * t35109;
    let t35112 = t4391 * t2365 * t25730;
    let t35113 = 0.59584149919750711116e-1_f64 * t35112;
    let t35115 = t4391 * t2365 * t25580;
    let t35116 = 0.29792074959875355558e-1_f64 * t35115;
    let t35119 = t31047 * t20671 * t27007;
    (t35104, t35110, t35113, t35116, t35119)
}
