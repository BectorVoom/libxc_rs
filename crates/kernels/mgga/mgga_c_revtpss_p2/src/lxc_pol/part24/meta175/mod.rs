//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta175 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk864;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk865;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk866;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk867;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk868;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk869;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk870;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta175<F: Float>(t1390: F, t6874: F, t828: F, t4012: F, t6836: F, t124: F, t6816: F, t800: F, t1370: F, t1388: F, t1410: F, t3934: F, t3976: F, t3987: F, t4002: F, t4064: F, t5611: F, t5619: F, t5623: F, t6864: F, t6871: F, t3944: F, t3950: F, t3956: F, t3967: F, t5606: F, t5625: F, t5666: F, t5681: F, t6846: F, t6850: F, t6856: F, t225: F, t1903: F, t4076: F, t1437: F, t1883: F, t213: F, t4082: F, t4085: F, t4099: F, t4113: F, t4114: F, t546: F, t5738: F, t5742: F, t5761: F, t5765: F, t5767: F, t6844: F, t6862: F, t820: F, t1427: F, t1424: F, t1904: F, t3894: F, t3898: F, t3910: F, t3922: F, t5601: F, t5604: F, t561: F, t5715: F, t5719: F, t5723: F, t1343: F, t1450: F, t198: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t3828: F, t4147: F, t532: F, t6777: F, t6778: F, t6779: F, t6780: F, t6781: F, t6802: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6876, t6880, t6883, t6884, t6887) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk864::<F>(t1390, t6874, t828, t4012, t6836, t124, t6816, t800, t1370, t1388, t1410, t3934, t3976, t3987, t4002, t4064, t5611, t5619, t5623, t6864, t6871);
        let t6888 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk865::<F>(t1388, t1410, t3944, t3950, t3956, t3967, t5606, t5625, t5666, t5681, t6846, t6850, t6856, t6887);
        let (t6889, t6895) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk866::<F>(t225, t6888, t1903);
        let t6896 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk867::<F>(t4076, t6895);
        let t6918 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk868::<F>(t1437, t1883, t213, t4082, t4085, t4099, t4113, t4114, t546, t5738, t5742, t5761, t5765, t5767, t6844, t6862, t6874, t6888, t820);
        let t6919 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk869::<F>(t1427, t6918);
        let t6922 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk870::<F>(t1424, t1904, t213, t3894, t3898, t3910, t3922, t5601, t5604, t561, t5715, t5719, t5723, t6889, t6896, t6919);
        let t6929 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk871::<F>(t1343, t1450, t198, t2522, t2562, t2569, t2579, t2587, t3828, t4147, t532, t6777, t6778, t6779, t6780, t6781, t6802, t6816, t6836, t6922);
    (t6876, t6880, t6883, t6884, t6888, t6889, t6895, t6896, t6918, t6919, t6922, t6929)
}
