//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta199 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1257;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1258;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1259;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1260;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1261;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1262;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1263;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta199<F: Float>(t1678: F, t994: F, t1668: F, t73: F, t3095: F, t3092: F, t3093: F, t357: F, t1592: F, t1058: F, t1660: F, t1053: F, t1659: F, t225: F, t4743: F, t366: F, t1065: F, t2857: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t4778 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1257::<F>(t1678, t994);
        let t4781 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1258::<F>(t1668, t73);
        let t4782 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1259::<F>(t3095, t4781);
        let t4783 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1260::<F>(t3092, t4782);
        let t4786 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1261::<F>(t3093, t357);
        let t4787 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1262::<F>(t1592, t4786);
        let t4788 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1263::<F>(t3092, t4787);
        let (t4792, t4794, t4797, t4798, t4801) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1264::<F>(t1058, t1660, t1053, t1659, t225, t4743, t366, t1065, t2857);
    (t4778, t4781, t4782, t4783, t4786, t4787, t4788, t4792, t4794, t4797, t4798, t4801)
}
