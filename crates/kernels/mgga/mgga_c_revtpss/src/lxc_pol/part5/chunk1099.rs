//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1099/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1099<F: Float>(t15125: F, t15191: F, t4742: F, t993: F, t225: F, t366: F, t3224: F, t4845: F, t127: F, t371: F, t4852: F, t1025: F) -> (F, F, F, F, F, F, F) {
    let t15638 = F::cast_from(0.19755555555555555556e-1_f64) * t15125;
    let t15639 = F::cast_from(0.9877777777777777778e-2_f64) * t15191;
    let t15654 = t4742 * t993;
    let t15655 = t15654 * t225;
    let t15656 = t15655 * t366;
    let t15662 = F::cast_from(0.28582678745379824648e-3_f64) * t3224 * t4845;
    let t15666 = t371 * t127 * t4852;
    let t15668 = F::cast_from(0.28582678745379824648e-3_f64) * t1025 * t15666;
    (t15638, t15639, t15654, t15655, t15656, t15662, t15668)
}
