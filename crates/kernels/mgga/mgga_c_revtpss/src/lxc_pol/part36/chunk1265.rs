//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1265/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1265<F: Float>(t1518: F, t1936: F, t5891: F, t94978: F, t25823: F, t5915: F, t29694: F, t689: F, t93314: F, t29682: F, t92838: F, t93302: F) -> (F, F, F, F, F, F, F) {
    let t105823 = t1518 * t1936;
    let t105870 = t94978 * t5891;
    let t105878 = t25823 * t5915;
    let t105933 = t29694 * t689;
    let t105934 = t93314 * t105933;
    let t105936 = t29682 * t689;
    let t105937 = t92838 * t105936;
    let t105939 = t93302 * t105933;
    (t105823, t105870, t105878, t105934, t105936, t105937, t105939)
}
