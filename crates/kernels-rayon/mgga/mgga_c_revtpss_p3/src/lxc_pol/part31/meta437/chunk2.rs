//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1563/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1563(t1045: f64, t19836: f64, t3117: f64, t11866: f64, t11927: f64, t15716: f64, t15771: f64, t15774: f64, t15776: f64, t15817: f64, t1671: f64, t19819: f64, t19827: f64, t19831: f64, t3115: f64, t4831: f64, t4834: f64, t4869: f64, t4879: f64, t6273: f64) -> (f64, f64) {
    let t19837 = t19836 * t1045;
    let t19838 = t3117 * t19837;
    let t19841 = -t15771 - t15774 + 0.31758531939310916275e-3_f64 * t15776 + 0.28582678745379824648e-3_f64 * t4834 * t4831 - 0.12862205435420921092e-2_f64 * t15716 * t19819 + 0.42874018118069736972e-3_f64 * t15817 * t1671 + 0.42874018118069736972e-3_f64 * t4879 * t4869 - 0.14291339372689912324e-3_f64 * t19827 + 0.42874018118069736972e-3_f64 * t11927 * t19831 - 0.42874018118069736972e-3_f64 * t11866 * t6273 - 0.42874018118069736972e-3_f64 * t3115 * t19838;
    (t19838, t19841)
}
