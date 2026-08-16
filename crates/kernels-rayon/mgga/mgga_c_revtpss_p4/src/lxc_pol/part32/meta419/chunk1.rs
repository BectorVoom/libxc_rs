//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1458/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1458(t19680: f64, t4801: f64, t1042: f64, t1063: f64, t15668: f64, t15675: f64, t15707: f64, t19651: f64, t19659: f64, t19663: f64, t19668: f64, t19672: f64, t19677: f64, t3127: f64, t3169: f64, t4837: f64, t4875: f64, t6302: f64) -> f64 {
    let t19681 = t4801 * t19680;
    let t19682 = t1042 * t19681;
    let t19685 = -t15668 + 0.28582678745379824648e-3_f64 * t4837 * t19651 - 0.28582678745379824648e-3_f64 * t15707 * t4875 - 0.11433071498151929859e-2_f64 * t3169 * t6302 + 0.14291339372689912324e-3_f64 * t19659 - 0.14291339372689912324e-2_f64 * t1063 * t19663 - t15675 + 0.47637797908966374414e-3_f64 * t1063 * t19668 + 0.63517063878621832552e-3_f64 * t1063 * t19672 - 0.14291339372689912324e-3_f64 * t3127 * t19677 - 0.28582678745379824648e-3_f64 * t1063 * t19682;
    t19685
}
