//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2527/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2527(t1045: f64, t19497: f64, t3117: f64, t1043: f64, t11631: f64, t19450: f64, t4894: f64, t19501: f64, t4910: f64, t11274: f64, t11277: f64, t11789: f64, t11875: f64, t15684: f64, t15906: f64, t16081: f64, t19731: f64, t19738: f64, t19741: f64, t3091: f64, t3115: f64, t4896: f64, t4902: f64, t6308: f64, t6312: f64, t6339: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19744 = t19497 * t1045;
    let t19745 = t3117 * t19744;
    let t19748 = t11631 * t1043;
    let t19749 = t19450 * t19748;
    let t19750 = t3117 * t19749;
    let t19753 = t19450 * t4894;
    let t19754 = t3117 * t19753;
    let t19757 = t19501 * t4910;
    let t19758 = t3117 * t19757;
    let t19763 = 0.28582678745379824648e-3_f64 * t3091 * t19731 + t15684 + 0.42874018118069736972e-3_f64 * t11274 * t6308 - 0.21437009059034868486e-3_f64 * t11277 * t6312 + 0.85748036236139473944e-3_f64 * t19738 * t4896 - 0.42874018118069736972e-3_f64 * t19741 * t4902 - 0.21437009059034868486e-3_f64 * t3115 * t19745 + 0.12862205435420921092e-2_f64 * t16081 * t19750 - 0.12862205435420921092e-2_f64 * t15906 * t19754 + 0.21437009059034868486e-3_f64 * t11875 * t19758 + 0.42874018118069736972e-3_f64 * t11789 * t6339;
    (t19744, t19745, t19748, t19749, t19750, t19753, t19754, t19757, t19758, t19763)
}
