//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta837 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2964;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta837<F: Float>(t1412: F, t808: F, t13927: F, t48862: F, t1389: F, t14224: F, t46835: F, t13769: F, t2453: F, t547: F, t9794: F, t14230: F, t2735: F, t46801: F, t40763: F, t5609: F, t9793: F, t13830: F, t9775: F, t13826: F, t3989: F, t13937: F, t9962: F, t13991: F, t13999: F, t13786: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48863, t48865, t48868, t48872, t48876) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2964::<F>(t1412, t808, t13927, t48862, t1389, t14224, t46835, t13769, t2453, t547, t9794, t14230, t2735, t46801);
        let (t48879, t48881, t48888, t48892, t48900, t48902) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2965::<F>(t40763, t5609, t9793, t13830, t9775, t13826, t3989, t13937, t9962, t13991, t13999, t13786);
    (t48863, t48865, t48868, t48872, t48876, t48879, t48881, t48888, t48892, t48900, t48902)
}
