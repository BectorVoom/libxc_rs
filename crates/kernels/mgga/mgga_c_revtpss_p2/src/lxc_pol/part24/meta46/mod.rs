//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta46 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk315;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk316;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk317;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk318;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk319;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk320;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk321;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk322;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk323;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta46<F: Float>(t270: F, t283: F, t66: F, t342: F, t378: F, t384: F, t225: F, t359: F, t1032: F, t1035: F, t355: F, t357: F, t389: F, t268: F, t404: F, t900: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1065 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk315::<F>(t270, t283);
        let t1066 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk316::<F>(t1065, t66);
        let t1076 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk317::<F>(t342, t378);
        let (t1077, t1078, t1079) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk318::<F>(t384, t225);
        let t1082 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk319::<F>(t359, t378);
        let t1086 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk320::<F>(t1032, t1035);
        let t1087 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk321::<F>(t1086, t342);
        let t1089 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk322::<F>(t355, t357);
        let (t1102, t1118) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk323::<F>(t389, t268, t404, t900);
    (t1065, t1066, t1076, t1077, t1078, t1079, t1082, t1086, t1087, t1089, t1102, t1118)
}
