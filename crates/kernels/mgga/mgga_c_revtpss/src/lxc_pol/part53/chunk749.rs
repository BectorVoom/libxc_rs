//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 749/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk749<F: Float>(t2042: F, t2170: F, t573: F, t8609: F, t8613: F, t8616: F, t8771: F, t3140: F, t3736: F, t1276: F, t1243: F, t197: F, t532: F, t1450: F, t2033: F, t4146: F, t565: F) -> (F, F, F, F, F, F) {
    let t8773 = t2170 * t2042;
    let t8776 = t573 * t8771 + 3.0 * t8609 + t8613 + t8616 + 3.0 * t8773;
    let t8939 = t3140 * t3736;
    let t8944 = t3140 * t1276;
    let t8945 = t8944 * t1243;
    let t8995 = t197 * t532;
    let t8996 = t2033 * t1450;
    let t9593 = 1.0 / t4146 / t565;
    (t8776, t8939, t8945, t8995, t8996, t9593)
}
