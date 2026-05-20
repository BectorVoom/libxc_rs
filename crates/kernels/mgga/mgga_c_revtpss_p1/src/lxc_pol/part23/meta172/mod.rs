//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta172 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1031;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1032;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1033;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1034;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1035;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1036;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta172<F: Float>(t1015: F, t4186: F, t1012: F, t3147: F, t72: F, t3088: F, t3299: F, t1668: F, t3153: F, t1043: F, t3154: F, t3117: F, t3317: F, t357: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4886, t4887, t4890) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1031::<F>(t1015, t4186, t1012, t3147, t72);
        let t4891 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1032::<F>(t3088, t4890);
        let t4892 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1033::<F>(t3299, t4891);
        let t4893 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1034::<F>(t1668, t3153);
        let t4894 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1035::<F>(t1043, t3154);
        let (t4895, t4896, t4899) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1036::<F>(t4893, t4894, t3117, t3317, t4891);
        let t4900 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1037::<F>(t1043, t357);
    (t4886, t4887, t4890, t4891, t4892, t4893, t4894, t4895, t4896, t4899, t4900)
}
