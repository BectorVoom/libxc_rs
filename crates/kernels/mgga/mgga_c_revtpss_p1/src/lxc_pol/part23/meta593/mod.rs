//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2234;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta593<F: Float>(t1045: F, t23820: F, t373: F, t1042: F, t11632: F, t23641: F, t11250: F, t1668: F, t6244: F, t3117: F, t1469: F, t5825: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t23822, t23823, t23829, t23830, t23833, t23834, t23837) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2234::<F>(t1045, t23820, t373, t1042, t11632, t23641, t11250, t1668, t6244);
        let (t23838, t23839, t23842) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2235::<F>(t1045, t23837, t3117, t1469, t5825);
    (t23822, t23823, t23829, t23830, t23833, t23834, t23837, t23838, t23839, t23842)
}
