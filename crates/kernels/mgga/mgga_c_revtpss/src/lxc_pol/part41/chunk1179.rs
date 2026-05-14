//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1179/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1179<F: Float>(t15957: F, t6266: F, t3092: F, t16509: F, t4891: F, t16584: F, t1045: F, t19497: F, t3117: F, t1043: F, t11631: F, t19450: F, t4894: F, t19501: F, t4910: F, t11274: F, t11277: F, t11789: F, t11875: F, t15684: F, t15906: F, t16081: F, t3091: F, t3115: F, t4896: F, t4902: F, t6308: F, t6312: F, t6339: F) -> (F,) {
    let t19730 = t15957 * t6266;
    let t19731 = t3092 * t19730;
    let t19738 = t16509 * t4891;
    let t19741 = t16584 * t4891;
    let t19744 = t19497 * t1045;
    let t19745 = t3117 * t19744;
    let t19748 = t11631 * t1043;
    let t19749 = t19450 * t19748;
    let t19750 = t3117 * t19749;
    let t19753 = t19450 * t4894;
    let t19754 = t3117 * t19753;
    let t19757 = t19501 * t4910;
    let t19758 = t3117 * t19757;
    let t19763 = 0.28582678745379824648e-3 * t3091 * t19731 + t15684 + 0.42874018118069736972e-3 * t11274 * t6308 - 0.21437009059034868486e-3 * t11277 * t6312 + 0.85748036236139473944e-3 * t19738 * t4896 - 0.42874018118069736972e-3 * t19741 * t4902 - 0.21437009059034868486e-3 * t3115 * t19745 + 0.12862205435420921092e-2 * t16081 * t19750 - 0.12862205435420921092e-2 * t15906 * t19754 + 0.21437009059034868486e-3 * t11875 * t19758 + 0.42874018118069736972e-3 * t11789 * t6339;
    (t19763,)
}
