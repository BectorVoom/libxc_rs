//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1643;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1644;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1645;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1646;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1647;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta436<F: Float>(t12571: F, t3535: F, t1196: F, t3516: F, t3542: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43813: F, t43854: F, t43883: F, t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F, t422: F, t44087: F, t44096: F, t44100: F, t44103: F, t44106: F, t44108: F, t44111: F, t44114: F, t44122: F, t12485: F, t12500: F, t3497: F, t12243: F, t12415: F, t12248: F, t3427: F, t3436: F, t1149: F, t12358: F, t3384: F, t12357: F, t3433: F, t3435: F, t12227: F, t12230: F, t3385: F, t3386: F, t1130: F, t12393: F, t1151: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44984, t44987, t44999) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1643::<F>(t12571, t3535, t1196, t3516, t3542, t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877);
        let t45012 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1644::<F>(t43813, t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t45015, t45016) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1645::<F>(t422, t44999, t45012, t44087, t44096, t44100, t44103, t44106, t44108, t44111, t44114, t44122, t44984, t44987);
        let (t45021, t45023, t45026, t45029) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1646::<F>(t1196, t12485, t12500, t3497, t12243, t12415, t12248, t3427, t3436, t1149, t12358, t3384);
        let (t45033, t45037, t45040, t45043) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1647::<F>(t1149, t12357, t3433, t3435, t12227, t12230, t3385, t3427, t3386, t1130, t12393, t1151);
    (t44984, t44987, t45015, t45016, t45021, t45023, t45026, t45029, t45033, t45037, t45040, t45043)
}
