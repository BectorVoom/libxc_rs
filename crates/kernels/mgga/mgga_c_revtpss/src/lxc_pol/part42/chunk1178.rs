//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1178/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1178<F: Float>(t19680: F, t4801: F, t1042: F, t1063: F, t15668: F, t15675: F, t15707: F, t19651: F, t19659: F, t19663: F, t19668: F, t19672: F, t19677: F, t3127: F, t3169: F, t4837: F, t4875: F, t6302: F) -> (F,) {
    let t19681 = t4801 * t19680;
    let t19682 = t1042 * t19681;
    let t19685 = -t15668 + 0.28582678745379824648e-3 * t4837 * t19651 - 0.28582678745379824648e-3 * t15707 * t4875 - 0.11433071498151929859e-2 * t3169 * t6302 + 0.14291339372689912324e-3 * t19659 - 0.14291339372689912324e-2 * t1063 * t19663 - t15675 + 0.47637797908966374414e-3 * t1063 * t19668 + 0.63517063878621832552e-3 * t1063 * t19672 - 0.14291339372689912324e-3 * t3127 * t19677 - 0.28582678745379824648e-3 * t1063 * t19682;
    (t19685,)
}
