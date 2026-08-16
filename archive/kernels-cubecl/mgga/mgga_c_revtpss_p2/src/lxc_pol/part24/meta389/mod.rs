//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1296;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1297;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta389<F: Float>(t159: F, t2698: F, t1544: F, t1583: F, t1868: F, t1907: F, t1501: F, t1518: F, t26: F, t65: F, t9163: F, t99: F, t107: F, t9232: F, t2565: F, t702: F, t9305: F, t2576: F, t2585: F, t9274: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25273, t29598, t30122, t30138, t33127, t36227) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1296::<F>(t159, t2698, t1544, t1583, t1868, t1907, t1501, t1518, t26, t65, t9163, t99);
        let (t36415, t39419) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1297::<F>(t107, t9232, t2565, t702, t9305);
        let t39422 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1298::<F>(t2576, t2585, t9274);
    (t25273, t29598, t30122, t30138, t33127, t36227, t36415, t39419, t39422)
}
