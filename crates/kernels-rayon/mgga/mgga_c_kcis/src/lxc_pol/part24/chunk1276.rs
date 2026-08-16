//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1276/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1276(t100074: f64, t26955: f64, t28130: f64, t96742: f64, t96743: f64, t10819: f64, t1851: f64, t28135: f64, t96908: f64, t100360: f64, t15534: f64, t19396: f64, t19399: f64, t26960: f64, t28116: f64, t29123: f64, t5302: f64, t92657: f64, t93082: f64, t93222: f64, t97338: f64) -> f64 {
    let t100865 = t26955 * t100074;
    let t100868 = t96742 * t96743 * t28130;
    let t100871 = t10819 * t1851;
    let t100873 = t96908 * t100871 * t28135;
    let t100893 = -0.82448622685185185184e-4_f64 * t93082 * t29123 + 0.10306077835648148148e-4_f64 * t100865 + 0.13901041666666666667e-2_f64 * t26960 * t100868 + 0.13901041666666666667e-2_f64 * t26960 * t100873 + 0.7722800925925925926e-4_f64 * t93222 + 0.18550940104166666667e-3_f64 * t26955 * t100868 + 0.185671721767578125e-4_f64 * t92657 * t100873 + 0.2782641015625e-3_f64 * t26955 * t100873 + 0.92673611111111111112e-3_f64 * t26960 * t5302 * t97338 * t19396 + 0.92673611111111111112e-3_f64 * t26960 * t15534 * t28116 * t19399 - 0.46377350260416666667e-4_f64 * t26955 * t100360;
    t100893
}
