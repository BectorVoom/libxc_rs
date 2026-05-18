//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1259/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1259<F: Float>(t15648: F, t373: F, t371: F, t372: F, t4742: F, t993: F, t225: F, t366: F, t3224: F, t4845: F, t127: F, t4852: F) -> (F, F, F, F, F, F) {
    let t15649 = t373 * t15648;
    let t15651 = t371 * t372 * t15649;
    let t15654 = t4742 * t993;
    let t15655 = t15654 * t225;
    let t15656 = t15655 * t366;
    let t15662 = F::new(0.28582678745379824648e-3) * t3224 * t4845;
    let t15666 = t371 * t127 * t4852;
    (t15651, t15654, t15655, t15656, t15662, t15666)
}
