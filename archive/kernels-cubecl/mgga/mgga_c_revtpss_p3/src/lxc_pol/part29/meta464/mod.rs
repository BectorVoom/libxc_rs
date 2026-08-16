//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1718;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta464<F: Float>(t212: F, t7506: F, t1358: F, t689: F, t2097: F, t785: F, t2439: F, t2435: F, t7493: F, t26069: F, t26277: F, t26072: F, t7515: F, t25924: F, t4077: F, t2027: F, t213: F, t25921: F, t25930: F, t26294: F, t26295: F, t26302: F, t26305: F, t26309: F, t26335: F, t26338: F, t26343: F, t26347: F, t26351: F, t4078: F, t561: F, t7295: F, t7511: F, t7523: F, t7528: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t26354, t26355, t26356, t26358, t26359, t26361, t26363, t26365, t26366) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1718::<F>(t212, t7506, t1358, t689, t2097, t785, t2439, t2435, t7493, t26069, t26277, t26072, t7515);
        let (t26371, t26374) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1719::<F>(t2097, t25924, t4077, t2027, t213, t25921, t25930, t26294, t26295, t26302, t26305, t26309, t26335, t26338, t26343, t26347, t26351, t26356, t26361, t26363, t26365, t26366, t4078, t561, t7295, t7511, t7523, t7528);
    (t26354, t26355, t26356, t26358, t26359, t26361, t26363, t26365, t26366, t26371, t26374)
}
