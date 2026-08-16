//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1643;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1644;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1645;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1646;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1647;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta436(t12571: f64, t3535: f64, t1196: f64, t3516: f64, t3542: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64, t43813: f64, t43854: f64, t43883: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64, t422: f64, t44087: f64, t44096: f64, t44100: f64, t44103: f64, t44106: f64, t44108: f64, t44111: f64, t44114: f64, t44122: f64, t12485: f64, t12500: f64, t3497: f64, t12243: f64, t12415: f64, t12248: f64, t3427: f64, t3436: f64, t1149: f64, t12358: f64, t3384: f64, t12357: f64, t3433: f64, t3435: f64, t12227: f64, t12230: f64, t3385: f64, t3386: f64, t1130: f64, t12393: f64, t1151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44984, t44987, t44999) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1643(t12571, t3535, t1196, t3516, t3542, t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877);
        let t45012 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1644(t43813, t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t45015, t45016) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1645(t422, t44999, t45012, t44087, t44096, t44100, t44103, t44106, t44108, t44111, t44114, t44122, t44984, t44987);
        let (t45021, t45023, t45026, t45029) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1646(t1196, t12485, t12500, t3497, t12243, t12415, t12248, t3427, t3436, t1149, t12358, t3384);
        let (t45033, t45037, t45040, t45043) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1647(t1149, t12357, t3433, t3435, t12227, t12230, t3385, t3427, t3386, t1130, t12393, t1151);
    (t44984, t44987, t45015, t45016, t45021, t45023, t45026, t45029, t45033, t45037, t45040, t45043)
}
