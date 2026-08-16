//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1384;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1385;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta360<F: Float>(t2470: F, t5740: F, t4101: F, t1432: F, t5763: F, t1385: F, t5710: F, t1904: F, t3899: F, t689: F, t3920: F, t5603: F, t2435: F, t5718: F, t1893: F, t2453: F, t3908: F, t3895: F, t2439: F, t213: F, t1532: F, t2609: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14243, t14252, t14255, t14276, t14280) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1384::<F>(t2470, t5740, t4101, t1432, t5763, t1385, t5710, t1904, t3899, t689, t3920, t5603);
        let (t14290, t14294, t14297, t14299, t14312) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1385::<F>(t2435, t5718, t1893, t2453, t3908, t1904, t3895, t2439, t213, t5710, t1532, t2609);
    (t14243, t14252, t14255, t14276, t14280, t14290, t14294, t14297, t14299, t14312)
}
