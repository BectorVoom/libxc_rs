//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta981 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3311;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3312;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3313;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3314;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3315;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3316;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3317;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3318;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3319;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3320;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3321;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3322;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta981(t5977: f64, t860: f64, t231: f64, t2782: f64, t2783: f64, t18657: f64, t233: f64, t689: f64, t869: f64, t10069: f64, t18750: f64, t822: f64, t10657: f64, t14663: f64, t14972: f64, t18714: f64, t2646: f64, t39656: f64, t39731: f64, t4424: f64, t4514: f64, t4526: f64, t51470: f64, t51483: f64, t6017: f64, t6022: f64, t820: f64, t837: f64, t1568: f64, t4423: f64, t6041: f64, t786: f64, t2801: f64, t10943: f64, t14546: f64, t18525: f64, t18616: f64, t18681: f64, t2754: f64, t2815: f64, t40267: f64, t40271: f64, t40273: f64, t40294: f64, t4366: f64, t4494: f64, t4504: f64, t51505: f64, t51507: f64, t18689: f64, t2435: f64, t18688: f64, t2439: f64, t2777: f64, t14587: f64, t51548: f64, t18677: f64, t18699: f64, t40284: f64, t40303: f64, t40314: f64, t40316: f64, t40318: f64, t51512: f64, t14602: f64, t14961: f64, t1558: f64, t2482: f64, t4469: f64, t14520: f64, t14568: f64, t14524: f64, t51297: f64, t14547: f64, t2724: f64, t51519: f64, t51521: f64, t51523: f64, t51527: f64, t51531: f64, t62209: f64, t136: f64, t2457: f64, t39680: f64, t10073: f64, t18746: f64, t14502: f64, t1559: f64, t18632: f64, t51332: f64, t51535: f64, t51537: f64, t51541: f64, t51544: f64, t51546: f64, t51550: f64, t51553: f64, t51560: f64, t18742: f64, t2718: f64, t2811: f64, t51436: f64, t51564: f64, t51572: f64, t51576: f64, t51578: f64, t51587: f64, t61866: f64, t18729: f64, t2470: f64, t2798: f64, t2723: f64, t4503: f64, t6016: f64, t879: f64, t40922: f64, t40924: f64, t51598: f64, t51600: f64, t51603: f64, t51610: f64, t51614: f64, t51617: f64, t14563: f64, t14598: f64, t14600: f64, t676: f64, t10535: f64, t40938: f64, t51529: f64, t51621: f64, t51623: f64, t51628: f64, t51632: f64, t51635: f64, t51637: f64, t51642: f64, t62641: f64, t10542: f64, t18726: f64, t51646: f64, t51653: f64, t51657: f64, t51660: f64, t51668: f64, t51672: f64, t51676: f64, t51680: f64, t51682: f64, t62385: f64, t62612: f64, t40945: f64, t40948: f64, t40952: f64, t40954: f64, t40956: f64, t40958: f64, t51684: f64, t51686: f64, t51688: f64, t51696: f64, t51700: f64, t51703: f64, t51708: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62760, t62763, t62775, t62777, t62788) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3311(t5977, t860, t231, t2782, t2783, t18657, t233, t689, t869, t10069, t18750, t822);
        let t62792 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3312(t10657, t14663, t14972, t18714, t2646, t39656, t39731, t4424, t4514, t4526, t51470, t51483, t6017, t6022, t62760, t62763, t62775, t62777, t62788, t820, t837);
        let (t62803, t62825) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3313(t1568, t4423, t2783, t6041, t786, t2801, t10943, t14546, t18525, t18616, t18681, t2646, t2754, t2815, t40267, t40271, t40273, t40294, t4366, t4494, t4504, t4514, t51505, t51507, t62760, t820, t837);
        let t62856 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3314(t231, t2782, t2783, t62803, t18689, t2435, t18688, t2439, t2777, t14587, t51548, t10943, t14546, t18525, t18677, t18699, t2646, t40284, t40303, t40314, t40316, t40318, t4504, t4514, t51512, t62760);
        let (t62868, t62887) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3315(t14602, t14961, t1558, t2482, t4469, t14520, t14568, t14524, t51297, t2801, t4526, t14546, t14547, t18699, t2724, t4366, t4494, t4504, t51519, t51521, t51523, t51527, t51531, t62209);
        let t62912 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3316(t136, t2457, t39680, t6022, t10073, t18746, t14502, t1559, t18632, t4366, t4504, t51332, t51535, t51537, t51541, t51544, t51546, t51550, t51553, t51560, t62803, t820);
        let t62945 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3317(t10073, t18742, t10069, t18746, t2718, t6041, t231, t2782, t2783, t62868, t14546, t14547, t18677, t18681, t2646, t2724, t2754, t2811, t4494, t4514, t51436, t51564, t51572, t51576, t51578, t51587, t61866, t820);
        let t62973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3318(t18729, t2470, t2798, t2723, t2782, t4503, t62760, t2482, t6016, t879, t2801, t14502, t18699, t2754, t40922, t40924, t4424, t4514, t51598, t51600, t51603, t51610, t51614, t51617);
        let (t62983, t62987, t62992, t62999) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3319(t14563, t14568, t14598, t14600, t4423, t676, t14602, t2482, t2811, t6016, t10535, t136, t2457, t6017);
        let t63002 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3320(t40938, t4494, t4504, t4514, t51529, t51621, t51623, t51628, t51632, t51635, t51637, t51642, t62641, t62983, t62987, t62992, t62999, t837);
        let t63024 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3321(t10542, t18726, t14546, t14547, t18677, t4514, t51646, t51653, t51657, t51660, t51668, t51672, t51676, t51680, t51682, t62385, t62612, t820, t837, t879);
        let t63041 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3322(t40945, t40948, t40952, t40954, t40956, t40958, t4514, t51684, t51686, t51688, t51696, t51700, t51703, t51708, t62868, t837);
    (t62792, t62825, t62856, t62887, t62912, t62945, t62973, t63002, t63024, t63041)
}
