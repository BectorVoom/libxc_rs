//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1581/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1581<F: Float>(t22841: F, t2661: F, t74026: F, t9934: F, t14100: F, t22399: F, t5722: F, t74835: F, t1357: F, t23043: F, t689: F, t1364: F, t22965: F, t786: F) -> (F, F, F, F, F) {
    let t86274 = t2661 * t9934 * t74026 * t22841;
    let t86285 = t14100 * t22399;
    let t86296 = t74835 * t5722;
    let t86300 = t689 * t1357 * t23043;
    let t86311 = t786 * t22965 * t1364;
    (t86274, t86285, t86296, t86300, t86311)
}
