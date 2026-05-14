//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1131/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1131<F: Float>(t100074: F, t26955: F, t28130: F, t96742: F, t96743: F, t10819: F, t1851: F, t28135: F, t96908: F, t100360: F, t15534: F, t19396: F, t19399: F, t26960: F, t28116: F, t29123: F, t5302: F, t92657: F, t93082: F, t93222: F, t97338: F) -> (F,) {
    let t100865 = t26955 * t100074;
    let t100868 = t96742 * t96743 * t28130;
    let t100871 = t10819 * t1851;
    let t100873 = t96908 * t100871 * t28135;
    let t100893 = -0.82448622685185185184e-4 * t93082 * t29123 + 0.10306077835648148148e-4 * t100865 + 0.13901041666666666667e-2 * t26960 * t100868 + 0.13901041666666666667e-2 * t26960 * t100873 + 0.7722800925925925926e-4 * t93222 + 0.18550940104166666667e-3 * t26955 * t100868 + 0.185671721767578125e-4 * t92657 * t100873 + 0.2782641015625e-3 * t26955 * t100873 + 0.92673611111111111112e-3 * t26960 * t5302 * t97338 * t19396 + 0.92673611111111111112e-3 * t26960 * t15534 * t28116 * t19399 - 0.46377350260416666667e-4 * t26955 * t100360;
    (t100893,)
}
