//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1145;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1146;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1147;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta271(t225: f64, t8085: f64, t1903: f64, t2097: f64, t7296: f64, t1882: f64, t543: f64, t7301: f64, t545: f64, t2028: f64, t1904: f64, t2027: f64, t2103: f64, t213: f64, t561: f64, t7295: f64, t7495: f64, t7498: f64, t7511: f64, t7517: f64, t7519: f64, t7917: f64, t532: f64, t1450: f64, t2107: f64, t5542: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1911: f64, t2014: f64, t2052: f64, t2056: f64, t2089: f64, t2093: f64, t2108: f64, t4248: f64, t508: f64, t569: f64, t651: f64, t7359: f64, t7732: f64, t7898: f64, t7969: f64, t7978: f64, t7984: f64, t7988: f64, t8065: f64, t8075: f64, t8079: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8086, t8094) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1145(t225, t8085, t1903, t2097);
        let (t8095, t8099) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1146(t7296, t8094, t1882, t2097, t543);
        let (t8100, t8103, t8104, t8107) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1147(t7301, t8099, t545, t8085, t2028, t1904, t2027, t2103, t213, t561, t7295, t7495, t7498, t7511, t7517, t7519, t7917, t8086, t8095);
        let (t8108, t8109, t8111, t8113) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1148(t532, t8107, t1450, t2107, t5542, t118, t1502, t1519, t1843, t1911, t2014, t2052, t2056, t2089, t2093, t2108, t4248, t508, t569, t651, t7359, t7732, t7898, t7969, t7978, t7984, t7988, t8065, t8075, t8079);
    (t8086, t8094, t8095, t8099, t8100, t8103, t8104, t8107, t8108, t8109, t8111, t8113)
}
