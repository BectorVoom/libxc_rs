//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 890/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk890<F: Float>(t12998: F, t12974: F, t12959: F, t12962: F, t12965: F, t12967: F, t12971: F, t12985: F, t12989: F, t12993: F, t12995: F, t13002: F, t13005: F, t13010: F, t14776: F, t1537: F) -> (F,) {
    let t14784 = 0.46308888888888888888e0 * t12998;
    let t14785 = 0.16068111111111111111e1 * t12974;
    let t14791 = -0.309885e1 * t12959 + 0.20839e0 * t12962 - 0.62517e0 * t12965 - 0.41678000000000000001e0 * t12967 - 0.157790625e0 * t12971 + 0.3529725e1 * t12993 + 0.6311625e0 * t12995 - t14784 - t14785 - 0.104195e0 * t13002 + 0.62517e0 * t13005 + 0.264729375e1 * t13010 - 0.103295e1 * t12985 + 0.309885e1 * t12989;
    let t14792 = t14776 + t14791;
    let t14793 = t14792 * t1537;
    (t14793,)
}
