//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1146/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1146<F: Float>(t1614: F, t2942: F, t11354: F, t1600: F, t2881: F, t11358: F, t2880: F, t4606: F, t918: F, t2889: F, t4598: F, t2897: F, t4614: F, t1606: F, t2439: F, t4580: F, t689: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15104 = t1614 * t2942;
    let t15107 = t11354 * t1600;
    let t15108 = t15107 * t2881;
    let t15110 = t11358 * t1600;
    let t15111 = t15110 * t2881;
    let t15113 = t2880 * t4606;
    let t15114 = t15113 * t918;
    let t15116 = t4598 * t2889;
    let t15118 = t2897 * t4606;
    let t15119 = t15118 * t918;
    let t15121 = t4614 * t2889;
    let t15123 = t2439 * t1606;
    let t15125 = t689 * t4580;
    (t15104, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125)
}
