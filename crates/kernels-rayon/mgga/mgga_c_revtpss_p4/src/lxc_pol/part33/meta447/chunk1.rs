//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1633/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1633(t20759: f64, t3737: f64, t1248: f64, t1715: f64, t3604: f64, t17353: f64, t12712: f64, t6638: f64, t13033: f64, t13058: f64, t17211: f64, t17219: f64, t17227: f64, t17243: f64, t17258: f64, t17260: f64, t17351: f64, t17654: f64, t5270: f64, t5304: f64, t5381: f64, t6631: f64, t6635: f64) -> (f64, f64, f64, f64) {
    let t20760 = t3737 * t20759;
    let t20765 = t1715 * t1248;
    let t20766 = t3604 * t20765;
    let t20767 = t17353 * t20766;
    let t20770 = t12712 * t6638;
    let t20771 = t17353 * t20770;
    let t20782 = t17211 + t17219 - t17227 - 0.57165357490759649296e-3_f64 * t17654 * t20767 + 0.28582678745379824648e-3_f64 * t17351 * t20771 + 0.47637797908966374413e-3_f64 * t5381 * t5304 + 0.42874018118069736972e-3_f64 * t13033 * t6631 - 0.21437009059034868486e-3_f64 * t13058 * t6635 - t17243 + t17258 - t17260 - 0.57165357490759649296e-3_f64 * t5381 * t5270;
    (t20760, t20767, t20771, t20782)
}
