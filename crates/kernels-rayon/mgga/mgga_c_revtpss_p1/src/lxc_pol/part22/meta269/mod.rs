//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta269 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1645;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1646;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1647;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1648;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1649;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1650;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1651;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1652;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1653;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta269(t6883: f64, t800: f64, t1370: f64, t1388: f64, t1410: f64, t3934: f64, t3976: f64, t3987: f64, t4002: f64, t4064: f64, t5611: f64, t5619: f64, t5623: f64, t6864: f64, t6871: f64, t6876: f64, t6880: f64, t3944: f64, t3950: f64, t3956: f64, t3967: f64, t5606: f64, t5625: f64, t5666: f64, t5681: f64, t6846: f64, t6850: f64, t6856: f64, t225: f64, t1903: f64, t4076: f64, t1437: f64, t1883: f64, t213: f64, t4082: f64, t4085: f64, t4099: f64, t4113: f64, t4114: f64, t546: f64, t5738: f64, t5742: f64, t5761: f64, t5765: f64, t5767: f64, t6844: f64, t6862: f64, t6874: f64, t820: f64, t1427: f64, t1424: f64, t1904: f64, t3894: f64, t3898: f64, t3910: f64, t3922: f64, t5601: f64, t5604: f64, t561: f64, t5715: f64, t5719: f64, t5723: f64, t1343: f64, t1450: f64, t198: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t3828: f64, t4147: f64, t532: f64, t6777: f64, t6778: f64, t6779: f64, t6780: f64, t6781: f64, t6802: f64, t6816: f64, t6836: f64, t1868: f64, t5532: f64, t3854: f64, t3859: f64, t3862: f64, t3865: f64, t3867: f64, t3871: f64, t3873: f64, t4027: f64, t4035: f64, t4037: f64, t4042: f64, t4139: f64, t6827: f64, t6828: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t4248: f64, t508: f64, t511: f64, t569: f64, t5877: f64, t5884: f64, t5887: f64, t5921: f64, t651: f64, t6765: f64, t6773: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6884, t6887) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1645(t6883, t800, t1370, t1388, t1410, t3934, t3976, t3987, t4002, t4064, t5611, t5619, t5623, t6864, t6871, t6876, t6880);
        let t6888 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1646(t1388, t1410, t3944, t3950, t3956, t3967, t5606, t5625, t5666, t5681, t6846, t6850, t6856, t6887);
        let (t6889, t6895) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1647(t225, t6888, t1903);
        let t6896 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1648(t4076, t6895);
        let t6918 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1649(t1437, t1883, t213, t4082, t4085, t4099, t4113, t4114, t546, t5738, t5742, t5761, t5765, t5767, t6844, t6862, t6874, t6888, t820);
        let t6919 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1650(t1427, t6918);
        let t6922 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1651(t1424, t1904, t213, t3894, t3898, t3910, t3922, t5601, t5604, t561, t5715, t5719, t5723, t6889, t6896, t6919);
        let t6929 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1652(t1343, t1450, t198, t2522, t2562, t2569, t2579, t2587, t3828, t4147, t532, t6777, t6778, t6779, t6780, t6781, t6802, t6816, t6836, t6922);
        let (t6930, t6933) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1653(t1868, t5532, t3854, t3859, t3862, t3865, t3867, t3871, t3873, t4027, t4035, t4037, t4042, t4139, t6827, t6828);
        let (t6934, t6936) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1654(t6929, t6933, t118, t1502, t1519, t1843, t1847, t1911, t4248, t508, t511, t569, t5877, t5884, t5887, t5921, t651, t6765, t6773);
    (t6884, t6888, t6889, t6895, t6896, t6918, t6919, t6922, t6930, t6934, t6936)
}
