//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1351/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1351<F: Float>(t20759: F, t3737: F, t1248: F, t1715: F, t3604: F, t17353: F, t12712: F, t6638: F, t13033: F, t13058: F, t17211: F, t17219: F, t17227: F, t17243: F, t17258: F, t17260: F, t17351: F, t17654: F, t5270: F, t5304: F, t5381: F, t6631: F, t6635: F) -> (F, F) {
    let t20760 = t3737 * t20759;
    let t20765 = t1715 * t1248;
    let t20766 = t3604 * t20765;
    let t20767 = t17353 * t20766;
    let t20770 = t12712 * t6638;
    let t20771 = t17353 * t20770;
    let t20782 = t17211 + t17219 - t17227 - F::cast_from(0.57165357490759649296e-3_f64) * t17654 * t20767 + F::cast_from(0.28582678745379824648e-3_f64) * t17351 * t20771 + F::cast_from(0.47637797908966374413e-3_f64) * t5381 * t5304 + F::cast_from(0.42874018118069736972e-3_f64) * t13033 * t6631 - F::cast_from(0.21437009059034868486e-3_f64) * t13058 * t6635 - t17243 + t17258 - t17260 - F::cast_from(0.57165357490759649296e-3_f64) * t5381 * t5270;
    (t20760, t20782)
}
