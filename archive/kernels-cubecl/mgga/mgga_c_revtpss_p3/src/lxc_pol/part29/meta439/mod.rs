//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1649;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta439<F: Float>(t4003: F, t5658: F, t1448: F, t1868: F, t197: F, t531: F, t2013: F, t1450: F, t3889: F, t2242: F, t607: F, t640: F, t644: F, t77: F, t2315: F, t84: F, t2251: F, t603: F, t2259: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21990, t22496, t25081, t25082) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1649::<F>(t4003, t5658, t1448, t1868, t197, t531, t2013);
        let (t25089, t25102, t25110, t25114, t25117, t25120) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1650::<F>(t1450, t3889, t2242, t607, t640, t644, t77, t2315, t84, t2251, t603, t2259);
    (t21990, t22496, t25081, t25082, t25089, t25102, t25110, t25114, t25117, t25120)
}
