//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1138;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1139;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1140;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta276<F: Float>(t1312: F, t1518: F, t2055: F, t4248: F, t7359: F, t7889: F, t7969: F, t7983: F, t7488: F, t7900: F, t7499: F, t7501: F, t7502: F, t7504: F, t7904: F, t7906: F, t7908: F, t225: F, t1903: F, t2097: F, t7296: F, t1882: F, t543: F, t7301: F, t545: F, t2028: F, t1904: F, t2027: F, t2103: F, t213: F, t561: F, t7295: F, t7495: F, t7498: F, t7511: F, t7517: F, t7519: F, t7917: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t8075, t8079, t8085) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1138::<F>(t1312, t1518, t2055, t4248, t7359, t7889, t7969, t7983, t7488, t7900, t7499, t7501, t7502, t7504, t7904, t7906, t7908);
        let (t8086, t8094) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1139::<F>(t225, t8085, t1903, t2097);
        let (t8095, t8099) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1140::<F>(t7296, t8094, t1882, t2097, t543);
        let (t8100, t8103, t8104, t8107) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1141::<F>(t7301, t8099, t545, t8085, t2028, t1904, t2027, t2103, t213, t561, t7295, t7495, t7498, t7511, t7517, t7519, t7917, t8086, t8095);
    (t8075, t8079, t8085, t8086, t8094, t8095, t8099, t8100, t8103, t8104, t8107)
}
