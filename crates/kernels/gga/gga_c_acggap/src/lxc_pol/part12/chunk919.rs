//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 919/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk919<F: Float>(t1983: F, t7585: F, t7586: F, t930: F, t7832: F, t7839: F, t1098: F, t7614: F, t1108: F, t7746: F, t1086: F, t1113: F) -> (F, F, F, F, F, F) {
    let t30967 = t7585 * t7586 * t1983 * t930;
    let t30974 = t7839 * t7832;
    let t30976 = t7614 * t1098;
    let t30978 = t7746 * t1108;
    let t30980 = t7614 * t1086;
    let t30982 = t7746 * t1113;
    (t30967, t30974, t30976, t30978, t30980, t30982)
}
