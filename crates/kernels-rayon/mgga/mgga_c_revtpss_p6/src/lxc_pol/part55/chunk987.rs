//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 987/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk987(t3670: f64, t7623: f64, t2133: f64, t816: f64, t1224: f64, t65: f64, t5052: f64, t1266: f64, t1808: f64, t26821: f64, t26822: f64, t26832: f64, t26836: f64, t26852: f64, t26867: f64, t29031: f64, t29034: f64, t29037: f64, t5386: f64, t5407: f64) -> (f64, f64) {
    let t29040 = t3670 * t7623;
    let t29047 = t2133 * t816;
    let t29048 = t65 * t1224;
    let t29049 = t29048 * t5052;
    let t29052 = -t26821 + 0.28582678745379824648e-3_f64 * t26822 - t29031 / 864.0_f64 - 0.28582678745379824648e-3_f64 * t26832 - 0.19055119163586549765e-3_f64 * t29034 - t26836 / 864.0_f64 - 0.28582678745379824648e-3_f64 * t29037 * t1266 + 0.85748036236139473944e-3_f64 * t29040 * t5386 - 0.28582678745379824648e-3_f64 * t26852 * t1808 - 0.28582678745379824648e-3_f64 * t26867 * t5407 - t29047 * t29049 / 144.0_f64;
    (t29047, t29052)
}
