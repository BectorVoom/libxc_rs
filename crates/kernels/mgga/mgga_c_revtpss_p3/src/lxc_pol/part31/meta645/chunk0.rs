//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2105/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2105<F: Float>(t105928: F, t27382: F, t29694: F, t689: F, t93314: F, t29682: F, t92838: F, t93302: F, t1032: F, t6041: F, t867: F, t786: F) -> (F, F, F, F, F, F, F, F) {
    let t105930 = F::new(2.0) * t27382 * t105928;
    let t105933 = t29694 * t689;
    let t105934 = t93314 * t105933;
    let t105936 = t29682 * t689;
    let t105937 = t92838 * t105936;
    let t105939 = t93302 * t105933;
    let t105944 = t6041 * t1032;
    let t105945 = t105944 * t867;
    let t105946 = t786 * t105945;
    (t105930, t105934, t105936, t105937, t105939, t105944, t105945, t105946)
}
