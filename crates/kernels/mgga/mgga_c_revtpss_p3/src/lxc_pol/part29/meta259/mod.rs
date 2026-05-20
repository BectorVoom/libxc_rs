//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1057;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1058;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1059;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1060;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1061;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1062;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta259<F: Float>(t3: F, t5789: F, param_d: F, t116: F, t1518: F, t670: F, t117: F, t4292: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, t2242: F, t38: F, t644: F, t84: F, t77: F, t603: F, t607: F, t640: F, t76: F, t112: F, t624: F, t655: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5790, t5795) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1057::<F>(t3, t5789, param_d);
        let (t5801, t5802, t5805, t5808) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1058::<F>(t116, t1518, t670, t117, t4292, t1459, t1461, t1916, t1918, t572, t573, t5795);
        let t6954 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1059::<F>(t2242, t38);
        let t6960 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1060::<F>(t644, t84, t77);
        let t6963 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1061::<F>(t603, t607);
        let t6977 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1062::<F>(t640, t76);
        let (t6996, t6998) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1063::<F>(t112, t624, t655, t68);
    (t5790, t5795, t5801, t5802, t5805, t5808, t6954, t6960, t6963, t6977, t6996, t6998)
}
