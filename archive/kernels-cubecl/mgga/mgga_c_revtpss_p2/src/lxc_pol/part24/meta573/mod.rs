//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1754;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1755;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta573<F: Float>(t141: F, t3417: F, t89837: F, t1145: F, t89849: F, t89867: F, t89871: F, t89875: F, t43764: F, t89830: F, t6449: F, t3390: F, t6442: F, t43946: F, t68255: F, t81156: F, t81158: F, t89824: F, t89828: F, t89832: F, t89839: F, t89843: F, t89847: F, t89851: F, t89855: F, t43881: F, t56236: F, t68257: F, t68399: F, t81230: F, t81232: F, t81234: F, t81236: F, t89865: F, t89869: F, t89873: F, t89877: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90402, t90405, t90408, t90411, t90414, t90417, t90419, t90420) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1754::<F>(t141, t3417, t89837, t1145, t89849, t89867, t89871, t89875, t43764, t89830, t6449, t3390);
        let (t90422, t90423, t90437) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1755::<F>(t6442, t43946, t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855);
        let t90449 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1756::<F>(t43881, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
    (t90402, t90405, t90408, t90411, t90414, t90417, t90419, t90420, t90422, t90423, t90437, t90449)
}
