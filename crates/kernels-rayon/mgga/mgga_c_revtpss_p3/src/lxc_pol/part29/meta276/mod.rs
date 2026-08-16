//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1138;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1139;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1140;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta276(t1312: f64, t1518: f64, t2055: f64, t4248: f64, t7359: f64, t7889: f64, t7969: f64, t7983: f64, t7488: f64, t7900: f64, t7499: f64, t7501: f64, t7502: f64, t7504: f64, t7904: f64, t7906: f64, t7908: f64, t225: f64, t1903: f64, t2097: f64, t7296: f64, t1882: f64, t543: f64, t7301: f64, t545: f64, t2028: f64, t1904: f64, t2027: f64, t2103: f64, t213: f64, t561: f64, t7295: f64, t7495: f64, t7498: f64, t7511: f64, t7517: f64, t7519: f64, t7917: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8075, t8079, t8085) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1138(t1312, t1518, t2055, t4248, t7359, t7889, t7969, t7983, t7488, t7900, t7499, t7501, t7502, t7504, t7904, t7906, t7908);
        let (t8086, t8094) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1139(t225, t8085, t1903, t2097);
        let (t8095, t8099) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1140(t7296, t8094, t1882, t2097, t543);
        let (t8100, t8103, t8104, t8107) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1141(t7301, t8099, t545, t8085, t2028, t1904, t2027, t2103, t213, t561, t7295, t7495, t7498, t7511, t7517, t7519, t7917, t8086, t8095);
    (t8075, t8079, t8085, t8086, t8094, t8095, t8099, t8100, t8103, t8104, t8107)
}
