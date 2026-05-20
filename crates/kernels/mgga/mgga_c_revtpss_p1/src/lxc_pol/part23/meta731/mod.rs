//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2500;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta731<F: Float>(t1531: F, t36: F, t14362: F, t9863: F, t9866: F, t2609: F, t4395: F, t10115: F, t1570: F, t11007: F, t1579: F, t4322: F, t9292: F, t10981: F, t22: F, t868: F, t15060: F, t2435: F, t14982: F, t2465: F, t2470: F, t4480: F, t9288: F, t1569: F, t2769: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50089, t50092, t50094, t50098, t50155, t50161, t50166) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2500::<F>(t1531, t36, t14362, t9863, t9866, t2609, t4395, t10115, t1570, t11007, t1579, t4322, t9292);
        let (t50178, t50184, t50187, t50205, t50208) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2501::<F>(t10981, t1579, t22, t868, t15060, t2435, t14982, t2465, t2470, t4480, t9288, t1569, t2769, t786);
    (t50089, t50092, t50094, t50098, t50155, t50161, t50166, t50178, t50184, t50187, t50205, t50208)
}
