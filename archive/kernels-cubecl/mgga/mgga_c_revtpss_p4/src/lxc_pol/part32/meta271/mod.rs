//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1145;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1146;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1147;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta271<F: Float>(t225: F, t8085: F, t1903: F, t2097: F, t7296: F, t1882: F, t543: F, t7301: F, t545: F, t2028: F, t1904: F, t2027: F, t2103: F, t213: F, t561: F, t7295: F, t7495: F, t7498: F, t7511: F, t7517: F, t7519: F, t7917: F, t532: F, t1450: F, t2107: F, t5542: F, t118: F, t1502: F, t1519: F, t1843: F, t1911: F, t2014: F, t2052: F, t2056: F, t2089: F, t2093: F, t2108: F, t4248: F, t508: F, t569: F, t651: F, t7359: F, t7732: F, t7898: F, t7969: F, t7978: F, t7984: F, t7988: F, t8065: F, t8075: F, t8079: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t8086, t8094) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1145::<F>(t225, t8085, t1903, t2097);
        let (t8095, t8099) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1146::<F>(t7296, t8094, t1882, t2097, t543);
        let (t8100, t8103, t8104, t8107) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1147::<F>(t7301, t8099, t545, t8085, t2028, t1904, t2027, t2103, t213, t561, t7295, t7495, t7498, t7511, t7517, t7519, t7917, t8086, t8095);
        let (t8108, t8109, t8111, t8113) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1148::<F>(t532, t8107, t1450, t2107, t5542, t118, t1502, t1519, t1843, t1911, t2014, t2052, t2056, t2089, t2093, t2108, t4248, t508, t569, t651, t7359, t7732, t7898, t7969, t7978, t7984, t7988, t8065, t8075, t8079);
    (t8086, t8094, t8095, t8099, t8100, t8103, t8104, t8107, t8108, t8109, t8111, t8113)
}
