//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2235;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2236;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta501<F: Float>(t1082: F, t15648: F, t3291: F, t4757: F, t3059: F, t5004: F, t16426: F, t3318: F, t1043: F, t1089: F, t4930: F, t15717: F, t3286: F, t4746: F, t1071: F, t3316: F, t342: F, t1647: F, t3298: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16479, t16482, t16485, t16488, t16496, t16499) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2235::<F>(t1082, t15648, t3291, t4757, t3059, t5004, t16426, t3318, t1043, t1089, t4930, t15717);
        let t16502 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2236::<F>(t3286, t4746);
        let (t16505, t16506, t16509) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2237::<F>(t1071, t3316, t342, t1647, t3298);
    (t16479, t16482, t16485, t16488, t16496, t16499, t16502, t16505, t16506, t16509)
}
