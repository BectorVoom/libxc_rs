//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk903;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk904;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta235<F: Float>(t247: F, t6326: F, t1066: F, t6096: F, t373: F, t6244: F, t371: F, t372: F, t1041: F, t1063: F, t1671: F, t1675: F, t3150: F, t3161: F, t3203: F, t3205: F, t375: F, t4834: F, t4846: F, t4879: F, t4925: F, t6302: F, t6308: F, t6312: F, t6318: F, t6323: F, t6298: F, t225: F, t385: F, t1695: F, t3269: F, t1082: F, t1089: F, t6271: F, t1651: F, t5004: F, t6258: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6327, t6331, t6337, t6339, t6342) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk903::<F>(t247, t6326, t1066, t6096, t373, t6244, t371, t372, t1041, t1063, t1671, t1675, t3150, t3161, t3203, t3205, t375, t4834, t4846, t4879, t4925, t6302, t6308, t6312, t6318, t6323);
        let t6343 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk904::<F>(t6298, t6342);
        let (t6345, t6350, t6351, t6362, t6365, t6368, t6371) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk905::<F>(t225, t385, t6343, t1695, t3269, t1082, t6244, t1089, t6271, t1651, t5004, t6258);
    (t6327, t6331, t6337, t6339, t6343, t6345, t6350, t6351, t6362, t6365, t6368, t6371)
}
