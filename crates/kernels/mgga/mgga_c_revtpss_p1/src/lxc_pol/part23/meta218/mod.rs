//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta218 (260520-c91 hierarchical CSE).
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
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1281;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1282;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1283;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1284;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1285;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1286;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1287;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1288;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1289;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta218<F: Float>(t1651: F, t996: F, t1695: F, t1079: F, t3070: F, t4571: F, t6094: F, t6098: F, t6102: F, t1592: F, t4823: F, t1042: F, t1469: F, t3094: F, t4781: F, t3092: F, t1668: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t6244 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1281::<F>(t1651);
        let t6245 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1282::<F>(t6244, t996);
        let t6251 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1283::<F>(t1651, t1695, t1079);
        let t6258 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1284::<F>(t3070, t4571, t6094, t6098, t6102);
        let t6259 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1285::<F>(t6258, t996);
        let (t6262, t6263) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1286::<F>(t1592, t4823, t1042);
        let t6266 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1287::<F>(t1469, t3094);
        let t6267 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1288::<F>(t4781, t6266);
        let t6268 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1289::<F>(t3092, t6267);
        let t6271 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1290::<F>(t1651, t1668);
    (t6244, t6245, t6251, t6258, t6259, t6262, t6263, t6266, t6267, t6268, t6271)
}
