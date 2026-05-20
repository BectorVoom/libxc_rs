//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3377/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3377<F: Float>(t15220: F, t4598: F, t18984: F, t2889: F, t18987: F, t4614: F, t18992: F, t18950: F, t2880: F, t918: F, t2897: F, t2881: F, t41401: F, t6113: F) -> (F, F, F, F, F, F, F, F) {
    let t63474 = t4598 * t15220;
    let t63476 = t18984 * t2889;
    let t63478 = t18987 * t2889;
    let t63480 = t4614 * t15220;
    let t63482 = t18992 * t2889;
    let t63485 = t2880 * t18950 * t918;
    let t63488 = t2897 * t18950 * t918;
    let t63491 = t41401 * t6113 * t2881;
    (t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491)
}
