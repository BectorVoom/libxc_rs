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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta382<F: Float>(t10293: F, t240: F, t243: F, t813: F, t816: F, t10675: F, t2689: F, t10777: F, t10779: F, t2706: F, t837: F, t798: F, t9726: F, t802: F, t10899: F, t794: F, t10902: F, t159: F, t216: F, t2475: F, t10764: F, t10770: F, t10771: F, t10785: F, t10786: F, t124: F, t2646: F, t2724: F, t2745: F, t2747: F, t2754: F, t39476: F, t40232: F, t40446: F, t40569: F, t40816: F, t40822: F, t40824: F, t40836: F, t40838: F, t40840: F, t4362: F, t4364: F, t4366: F, t799: F, t800: F, t40392: F, t40457: F, t40520: F, t40596: F, t40671: F, t40746: F, t40811: F, t10661: F, t10861: F, t213: F, t234: F, t39714: F, t40298: F, t40303: F, t40307: F, t40311: F, t40314: F, t40316: F, t40318: F, t40369: F, t4504: F, t820: F, t879: F, t2645: F, t860: F, t231: F, t2782: F, t2783: F, t251: F, t40321: F, t2723: F, t39704: F, t4503: F, t123: F, t212: F, t9291: F, t2786: F, t10073: F, t10654: F, t10666: F, t10952: F, t2815: F, t40326: F, t40491: F, t40537: F, t4514: F, t10910: F, t822: F, t10959: F, t2439: F, t2777: F, t686: F, t72: F, t874: F, t10914: F, t2710: F, t9285: F, t10972: F, t2470: F, t136: F, t2457: F, t2760: F, t10929: F, t10069: F, t2790: F, t9292: F, t10932: F, t2811: F, t40340: F, t2444: F, t2829: F, t689: F, t11003: F, t9303: F, t10978: F, t779: F, t10652: F, t10872: F, t10943: F, t10977: F, t14546: F, t2770: F, t39549: F, t39550: F, t39554: F, t39557: F, t39558: F, t39562: F, t39565: F, t39567: F, t39570: F, t39573: F, t39576: F, t39581: F, t39586: F, t39590: F, t39595: F, t39602: F, t39606: F, t39610: F, t39612: F, t39617: F, t39664: F, t39668: F, t39673: F, t39678: F, t39683: F, t39685: F, t39687: F, t39692: F, t39694: F, t39697: F, t39701: F, t40255: F, t40258: F, t40263: F, t40267: F, t40271: F, t40273: F, t40278: F, t40282: F, t40284: F, t40294: F, t865: F, t868: F, t886: F, t10981: F, t22: F, t780: F, t10988: F, t2435: F, t2445: F) -> (F, F, F, F, F, F, F, F) {
        let (t40846, t40850, t40851, t40855, t40861) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1388::<F>(t10293, t240, t243, t813, t816, t10675, t2689, t10777, t10779, t2706, t837, t798, t9726);
        let t40873 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1389::<F>(t40861, t802, t10899, t794, t10902, t159, t216, t2475, t10764, t10770, t10771, t10785, t10786, t124, t2646, t2724, t2745, t2747, t2754, t39476, t40232, t40446, t40569, t40816, t40822, t40824, t40836, t40838, t40840, t40850, t40851, t40855, t4362, t4364, t4366, t799, t800);
        let (t40876, t40886) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1390::<F>(t40392, t40457, t40520, t40596, t40671, t40746, t40811, t40873, t10661, t10861, t213, t234, t39714, t40298, t40303, t40307, t40311, t40314, t40316, t40318, t40369, t4366, t4504, t820, t879);
        let (t40888, t40894, t40902, t40914, t40918, t40921) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1391::<F>(t2645, t860, t231, t2782, t2783, t39714, t251, t40321, t2723, t39704, t4503, t123, t212, t9291);
        let t40926 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1392::<F>(t2786, t40921, t10073, t10654, t10666, t10952, t2815, t40326, t40491, t40537, t40888, t40894, t40902, t40914, t40918, t4366, t4504, t4514, t820, t837, t879);
        let (t40927, t40938, t40942, t40945, t40948) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1393::<F>(t10910, t822, t10959, t2439, t2777, t686, t72, t874, t10914, t2710, t9285, t10972, t2470);
        let t40960 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1394::<F>(t136, t2457, t2710, t2760, t10073, t10929, t10069, t10654, t2790, t9292, t10932, t2754, t2811, t40340, t40927, t40938, t40942, t40945, t40948, t4514, t820, t837);
        let t40975 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1395::<F>(t2444, t2829, t689, t11003, t9303, t10978, t779, t10652, t10872, t10943, t10977, t14546, t2770, t2811, t39549, t39550, t39554, t39557, t39558, t39562, t39565, t39567, t39570, t39573, t39576, t39581, t39586, t39590, t39595, t39602, t39606, t39610, t39612, t39617, t39664, t39668, t39673, t39678, t39683, t39685, t39687, t39692, t39694, t39697, t39701, t40255, t40258, t40263, t40267, t40271, t40273, t40278, t40282, t40284, t40294, t40886, t40926, t40960, t4504, t820, t865, t868, t886);
        let (t40978, t40982, t40986, t40988) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1396::<F>(t10981, t22, t868, t886, t10910, t212, t689, t780, t10988, t2435, t2445, t9292);
    (t40846, t40876, t40921, t40975, t40978, t40982, t40986, t40988)
}
