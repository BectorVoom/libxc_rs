//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta874 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2776;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta874<F: Float>(t22126: F, t2689: F, t22130: F, t22081: F, t9962: F, t22276: F, t3989: F, t22281: F, t22056: F, t9765: F, t22021: F, t808: F, t9845: F, t22041: F, t3957: F, t2661: F, t74026: F, t9835: F, t9934: F, t22016: F, t22025: F, t46609: F, t6846: F, t9909: F, t1399: F, t22236: F, t3992: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74491, t74493, t74498, t74505, t74507, t74511, t74522) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2776::<F>(t22126, t2689, t22130, t22081, t9962, t22276, t3989, t22281, t22056, t9765, t22021, t808, t9845);
        let (t74547, t74579, t74583, t74585, t74589) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2777::<F>(t22041, t3957, t2661, t74026, t9835, t9934, t22016, t22025, t46609, t6846, t9909, t1399, t22236, t3992);
    (t74491, t74493, t74498, t74505, t74507, t74511, t74522, t74547, t74579, t74583, t74585, t74589)
}
