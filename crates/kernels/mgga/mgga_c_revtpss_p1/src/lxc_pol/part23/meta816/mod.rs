//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta816 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2662;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2663;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta816<F: Float>(t11921: F, t15716: F, t19456: F, t247: F, t19696: F, t3168: F, t15830: F, t4817: F, t1063: F, t11986: F, t6100: F, t20054: F, t3106: F, t19701: F, t3127: F, t3172: F, t19658: F, t3169: F, t19894: F, t15707: F, t15734: F, t19882: F, t3188: F, t16190: F, t4820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t65298, t65342, t65347, t65357, t65359) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2662::<F>(t11921, t15716, t19456, t247, t19696, t3168, t15830, t4817, t1063, t11986, t6100, t20054, t3106);
        let (t65376, t65431, t65444, t65446, t65454, t65456) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2663::<F>(t19701, t3127, t3172, t19658, t3169, t19894, t15707, t15734, t19882, t3188, t16190, t4820);
    (t65298, t65342, t65347, t65357, t65359, t65376, t65431, t65444, t65446, t65454, t65456)
}
