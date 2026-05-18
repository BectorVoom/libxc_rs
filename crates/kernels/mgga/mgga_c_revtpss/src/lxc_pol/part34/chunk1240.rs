//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1240/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1240<F: Float>(t105933: F, t93314: F, t29682: F, t689: F, t92838: F, t93302: F, t1032: F, t6041: F, t867: F, t786: F, t7060: F, t92843: F) -> (F, F, F, F, F, F, F) {
    let t105934 = t93314 * t105933;
    let t105936 = t29682 * t689;
    let t105937 = t92838 * t105936;
    let t105939 = t93302 * t105933;
    let t105944 = t6041 * t1032;
    let t105945 = t105944 * t867;
    let t105946 = t786 * t105945;
    let t105947 = t105946 * t7060;
    let t105949 = t92843 * t105936;
    (t105934, t105937, t105939, t105944, t105945, t105947, t105949)
}
