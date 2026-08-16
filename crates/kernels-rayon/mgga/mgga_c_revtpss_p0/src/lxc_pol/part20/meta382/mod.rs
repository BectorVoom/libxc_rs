//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta382 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1388;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1389;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1390;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1391;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1392;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1393;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1394;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1395;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta382(t10293: f64, t240: f64, t243: f64, t813: f64, t816: f64, t10675: f64, t2689: f64, t10777: f64, t10779: f64, t2706: f64, t837: f64, t798: f64, t9726: f64, t802: f64, t10899: f64, t794: f64, t10902: f64, t159: f64, t216: f64, t2475: f64, t10764: f64, t10770: f64, t10771: f64, t10785: f64, t10786: f64, t124: f64, t2646: f64, t2724: f64, t2745: f64, t2747: f64, t2754: f64, t39476: f64, t40232: f64, t40446: f64, t40569: f64, t40816: f64, t40822: f64, t40824: f64, t40836: f64, t40838: f64, t40840: f64, t4362: f64, t4364: f64, t4366: f64, t799: f64, t800: f64, t40392: f64, t40457: f64, t40520: f64, t40596: f64, t40671: f64, t40746: f64, t40811: f64, t10661: f64, t10861: f64, t213: f64, t234: f64, t39714: f64, t40298: f64, t40303: f64, t40307: f64, t40311: f64, t40314: f64, t40316: f64, t40318: f64, t40369: f64, t4504: f64, t820: f64, t879: f64, t2645: f64, t860: f64, t231: f64, t2782: f64, t2783: f64, t251: f64, t40321: f64, t2723: f64, t39704: f64, t4503: f64, t123: f64, t212: f64, t9291: f64, t2786: f64, t10073: f64, t10654: f64, t10666: f64, t10952: f64, t2815: f64, t40326: f64, t40491: f64, t40537: f64, t4514: f64, t10910: f64, t822: f64, t10959: f64, t2439: f64, t2777: f64, t686: f64, t72: f64, t874: f64, t10914: f64, t2710: f64, t9285: f64, t10972: f64, t2470: f64, t136: f64, t2457: f64, t2760: f64, t10929: f64, t10069: f64, t2790: f64, t9292: f64, t10932: f64, t2811: f64, t40340: f64, t2444: f64, t2829: f64, t689: f64, t11003: f64, t9303: f64, t10978: f64, t779: f64, t10652: f64, t10872: f64, t10943: f64, t10977: f64, t14546: f64, t2770: f64, t39549: f64, t39550: f64, t39554: f64, t39557: f64, t39558: f64, t39562: f64, t39565: f64, t39567: f64, t39570: f64, t39573: f64, t39576: f64, t39581: f64, t39586: f64, t39590: f64, t39595: f64, t39602: f64, t39606: f64, t39610: f64, t39612: f64, t39617: f64, t39664: f64, t39668: f64, t39673: f64, t39678: f64, t39683: f64, t39685: f64, t39687: f64, t39692: f64, t39694: f64, t39697: f64, t39701: f64, t40255: f64, t40258: f64, t40263: f64, t40267: f64, t40271: f64, t40273: f64, t40278: f64, t40282: f64, t40284: f64, t40294: f64, t865: f64, t868: f64, t886: f64, t10981: f64, t22: f64, t780: f64, t10988: f64, t2435: f64, t2445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40846, t40850, t40851, t40855, t40861) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1388(t10293, t240, t243, t813, t816, t10675, t2689, t10777, t10779, t2706, t837, t798, t9726);
        let t40873 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1389(t40861, t802, t10899, t794, t10902, t159, t216, t2475, t10764, t10770, t10771, t10785, t10786, t124, t2646, t2724, t2745, t2747, t2754, t39476, t40232, t40446, t40569, t40816, t40822, t40824, t40836, t40838, t40840, t40850, t40851, t40855, t4362, t4364, t4366, t799, t800);
        let (t40876, t40886) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1390(t40392, t40457, t40520, t40596, t40671, t40746, t40811, t40873, t10661, t10861, t213, t234, t39714, t40298, t40303, t40307, t40311, t40314, t40316, t40318, t40369, t4366, t4504, t820, t879);
        let (t40888, t40894, t40902, t40914, t40918, t40921) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1391(t2645, t860, t231, t2782, t2783, t39714, t251, t40321, t2723, t39704, t4503, t123, t212, t9291);
        let t40926 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1392(t2786, t40921, t10073, t10654, t10666, t10952, t2815, t40326, t40491, t40537, t40888, t40894, t40902, t40914, t40918, t4366, t4504, t4514, t820, t837, t879);
        let (t40927, t40938, t40942, t40945, t40948) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1393(t10910, t822, t10959, t2439, t2777, t686, t72, t874, t10914, t2710, t9285, t10972, t2470);
        let t40960 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1394(t136, t2457, t2710, t2760, t10073, t10929, t10069, t10654, t2790, t9292, t10932, t2754, t2811, t40340, t40927, t40938, t40942, t40945, t40948, t4514, t820, t837);
        let t40975 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1395(t2444, t2829, t689, t11003, t9303, t10978, t779, t10652, t10872, t10943, t10977, t14546, t2770, t2811, t39549, t39550, t39554, t39557, t39558, t39562, t39565, t39567, t39570, t39573, t39576, t39581, t39586, t39590, t39595, t39602, t39606, t39610, t39612, t39617, t39664, t39668, t39673, t39678, t39683, t39685, t39687, t39692, t39694, t39697, t39701, t40255, t40258, t40263, t40267, t40271, t40273, t40278, t40282, t40284, t40294, t40886, t40926, t40960, t4504, t820, t865, t868, t886);
        let (t40978, t40982, t40986, t40988) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1396(t10981, t22, t868, t886, t10910, t212, t689, t780, t10988, t2435, t2445, t9292);
    (t40846, t40876, t40921, t40975, t40978, t40982, t40986, t40988)
}
