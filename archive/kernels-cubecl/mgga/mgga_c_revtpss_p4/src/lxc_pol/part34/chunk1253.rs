//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1253/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1253<F: Float>(t20020: F, t7117: F, t19907: F, t7111: F, t19912: F, t27479: F, t4845: F, t1035: F, t29807: F, t29834: F, t7166: F, t1976: F, t6305: F) -> (F, F, F, F, F, F, F) {
    let t107140 = t7117 * t20020;
    let t107154 = t7111 * t19907;
    let t107169 = t7111 * t19912;
    let t107188 = t27479 * t4845;
    let t107207 = t1035 * t29807;
    let t107212 = t29834 * t7166;
    let t107225 = t1976 * t6305;
    (t107140, t107154, t107169, t107188, t107207, t107212, t107225)
}
