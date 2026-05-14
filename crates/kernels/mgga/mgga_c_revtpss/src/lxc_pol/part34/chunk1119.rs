//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1119/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1119<F: Float>(t20016: F, t25500: F, t19463: F, t1972: F, t19976: F, t25580: F, t19900: F, t7111: F, t1058: F, t29779: F, t20020: F, t7117: F, t19907: F, t19912: F, t27479: F, t4845: F) -> (F, F, F, F, F, F, F, F, F) {
    let t107064 = t25500 * t20016;
    let t107072 = t19463 * t1972;
    let t107086 = t25580 * t19976;
    let t107101 = t7111 * t19900;
    let t107107 = t29779 * t1058;
    let t107140 = t7117 * t20020;
    let t107154 = t7111 * t19907;
    let t107169 = t7111 * t19912;
    let t107188 = t27479 * t4845;
    (t107064, t107072, t107086, t107101, t107107, t107140, t107154, t107169, t107188)
}
