//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1564/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1564<F: Float>(t15957: F, t6266: F, t3092: F, t16509: F, t4891: F, t16584: F, t1045: F, t19497: F, t3117: F, t1043: F, t11631: F, t19450: F) -> (F, F, F, F, F) {
    let t19730 = t15957 * t6266;
    let t19731 = t3092 * t19730;
    let t19738 = t16509 * t4891;
    let t19741 = t16584 * t4891;
    let t19744 = t19497 * t1045;
    let t19745 = t3117 * t19744;
    let t19748 = t11631 * t1043;
    let t19749 = t19450 * t19748;
    (t19731, t19738, t19741, t19745, t19749)
}
