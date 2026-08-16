//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1393/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1393(t12784: f64, t12866: f64, t12910: f64, t17619: f64, t17622: f64, t17625: f64, t17629: f64, t17635: f64, t17640: f64, t17646: f64, t17651: f64, t17654: f64, t17658: f64, t17662: f64, t3625: f64, t5402: f64) -> f64 {
    let t17665 = -t17619 - t17622 + 0.42874018118069736972e-3_f64 * t12910 * t17625 + t17629 / 1296.0_f64 - 0.28582678745379824648e-3_f64 * t12784 * t5402 - 0.28582678745379824648e-3_f64 * t3625 * t17635 - 0.14291339372689912324e-3_f64 * t3625 * t17640 - 0.28582678745379824648e-3_f64 * t3625 * t17646 + 0.28582678745379824648e-3_f64 * t12866 * t17651 - 0.57165357490759649296e-3_f64 * t17654 * t17658 + 0.28582678745379824648e-3_f64 * t12866 * t17662;
    t17665
}
