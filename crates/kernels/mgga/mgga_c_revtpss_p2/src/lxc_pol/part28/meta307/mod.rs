//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1307;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1308;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta307<F: Float>(t1398: F, t215: F, t268: F, t543: F, t4101: F, t2453: F, t4100: F, t281: F, t68: F, t1357: F, t4078: F, t689: F, t1445: F, t3899: F, t10115: F, t562: F, t2435: F, t3903: F, t3895: F, t2439: F, t1420: F, t3908: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10137, t10139, t10143, t10151) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1307::<F>(t1398, t215, t268, t543, t4101, t2453, t4100, t281, t68, t1357, t4078, t689);
        let (t10154, t10157, t10160, t10163, t10166) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1308::<F>(t1445, t3899, t689, t10115, t562, t2435, t3903, t3895, t2439, t1420, t2453, t3908);
    (t10137, t10139, t10143, t10151, t10154, t10157, t10160, t10163, t10166)
}
