//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2075;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2076;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2077;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2078;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2079;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2080;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2081;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta537<F: Float>(t1882: F, t1892: F, t4003: F, t5658: F, t10032: F, t10035: F, t10044: F, t1399: F, t14116: F, t14120: F, t14126: F, t14131: F, t14146: F, t14149: F, t14158: F, t14161: F, t14166: F, t4118: F, t5735: F, t5745: F, t5755: F, t6844: F, t820: F, t555: F, t6861: F, t6843: F, t1398: F, t9994: F, t550: F, t543: F, t3992: F, t2661: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t21981 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2075::<F>(t1882, t1892);
        let t21990 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2076::<F>(t4003, t5658);
        let t21998 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2077::<F>(t10032, t10035, t10044, t1399, t14116, t14120, t14126, t14131, t14146, t14149, t14158, t14161, t14166, t21981, t21990, t4118, t5735, t5745, t5755, t6844, t820);
        let t22005 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2078::<F>(t555, t6861);
        let t22009 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2079::<F>(t555, t6843);
        let t22016 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2080::<F>(t1398, t9994);
        let t22020 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2081::<F>(t550, t6843);
        let (t22021, t22022, t22023, t22025) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2082::<F>(t22020, t543, t3992, t2661, t550, t6861);
    (t21981, t21990, t21998, t22005, t22009, t22016, t22020, t22021, t22022, t22023, t22025)
}
