//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta874 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2776;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta874(t22126: f64, t2689: f64, t22130: f64, t22081: f64, t9962: f64, t22276: f64, t3989: f64, t22281: f64, t22056: f64, t9765: f64, t22021: f64, t808: f64, t9845: f64, t22041: f64, t3957: f64, t2661: f64, t74026: f64, t9835: f64, t9934: f64, t22016: f64, t22025: f64, t46609: f64, t6846: f64, t9909: f64, t1399: f64, t22236: f64, t3992: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74491, t74493, t74498, t74505, t74507, t74511, t74522) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2776(t22126, t2689, t22130, t22081, t9962, t22276, t3989, t22281, t22056, t9765, t22021, t808, t9845);
        let (t74547, t74579, t74583, t74585, t74589) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2777(t22041, t3957, t2661, t74026, t9835, t9934, t22016, t22025, t46609, t6846, t9909, t1399, t22236, t3992);
    (t74491, t74493, t74498, t74505, t74507, t74511, t74522, t74547, t74579, t74583, t74585, t74589)
}
