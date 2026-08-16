//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1436/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1436<F: Float>(t15113: F, t918: F, t2889: F, t4598: F, t2897: F, t4606: F, t4614: F, t1606: F, t2439: F, t4580: F, t689: F) -> (F, F, F, F, F, F) {
    let t15114 = t15113 * t918;
    let t15116 = t4598 * t2889;
    let t15118 = t2897 * t4606;
    let t15119 = t15118 * t918;
    let t15121 = t4614 * t2889;
    let t15123 = t2439 * t1606;
    let t15125 = t689 * t4580;
    (t15114, t15116, t15119, t15121, t15123, t15125)
}
