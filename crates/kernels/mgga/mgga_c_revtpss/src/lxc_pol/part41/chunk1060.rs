//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1060/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1060<F: Float>(t3090: F, t4954: F, t15125: F, t15191: F, t4742: F, t993: F, t225: F, t366: F, t3224: F, t4845: F, t127: F, t371: F, t4852: F, t1025: F, t1646: F, t3056: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15618 = t4954 * t3090;
    let t15638 = 0.19755555555555555556e-1 * t15125;
    let t15639 = 0.9877777777777777778e-2 * t15191;
    let t15654 = t4742 * t993;
    let t15655 = t15654 * t225;
    let t15656 = t15655 * t366;
    let t15662 = 0.28582678745379824648e-3 * t3224 * t4845;
    let t15666 = t371 * t127 * t4852;
    let t15668 = 0.28582678745379824648e-3 * t1025 * t15666;
    let t15669 = t1646 * t3056;
    (t15618, t15638, t15639, t15654, t15655, t15656, t15662, t15668, t15669)
}
