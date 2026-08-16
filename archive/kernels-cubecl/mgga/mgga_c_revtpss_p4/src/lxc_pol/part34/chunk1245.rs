//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1245/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1245<F: Float>(t18352: F, t1945: F, t807: F, t29654: F, t686: F, t72: F, t25387: F, t25375: F, t29610: F, t29668: F, t689: F, t25431: F) -> (F, F, F, F, F, F, F) {
    let t106102 = t807 * t1945 * t18352;
    let t106120 = t29654 * t72 * t686;
    let t106121 = t25387 * t106120;
    let t106123 = t25375 * t106120;
    let t106128 = t29610 * t72 * t686;
    let t106129 = t25387 * t106128;
    let t106150 = t29668 * t689;
    let t106151 = t25431 * t106150;
    (t106102, t106121, t106123, t106128, t106129, t106150, t106151)
}
