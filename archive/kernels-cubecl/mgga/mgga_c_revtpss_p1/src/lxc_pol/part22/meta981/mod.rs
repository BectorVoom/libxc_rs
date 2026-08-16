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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta981<F: Float>(t5977: F, t860: F, t231: F, t2782: F, t2783: F, t18657: F, t233: F, t689: F, t869: F, t10069: F, t18750: F, t822: F, t10657: F, t14663: F, t14972: F, t18714: F, t2646: F, t39656: F, t39731: F, t4424: F, t4514: F, t4526: F, t51470: F, t51483: F, t6017: F, t6022: F, t820: F, t837: F, t1568: F, t4423: F, t6041: F, t786: F, t2801: F, t10943: F, t14546: F, t18525: F, t18616: F, t18681: F, t2754: F, t2815: F, t40267: F, t40271: F, t40273: F, t40294: F, t4366: F, t4494: F, t4504: F, t51505: F, t51507: F, t18689: F, t2435: F, t18688: F, t2439: F, t2777: F, t14587: F, t51548: F, t18677: F, t18699: F, t40284: F, t40303: F, t40314: F, t40316: F, t40318: F, t51512: F, t14602: F, t14961: F, t1558: F, t2482: F, t4469: F, t14520: F, t14568: F, t14524: F, t51297: F, t14547: F, t2724: F, t51519: F, t51521: F, t51523: F, t51527: F, t51531: F, t62209: F, t136: F, t2457: F, t39680: F, t10073: F, t18746: F, t14502: F, t1559: F, t18632: F, t51332: F, t51535: F, t51537: F, t51541: F, t51544: F, t51546: F, t51550: F, t51553: F, t51560: F, t18742: F, t2718: F, t2811: F, t51436: F, t51564: F, t51572: F, t51576: F, t51578: F, t51587: F, t61866: F, t18729: F, t2470: F, t2798: F, t2723: F, t4503: F, t6016: F, t879: F, t40922: F, t40924: F, t51598: F, t51600: F, t51603: F, t51610: F, t51614: F, t51617: F, t14563: F, t14598: F, t14600: F, t676: F, t10535: F, t40938: F, t51529: F, t51621: F, t51623: F, t51628: F, t51632: F, t51635: F, t51637: F, t51642: F, t62641: F, t10542: F, t18726: F, t51646: F, t51653: F, t51657: F, t51660: F, t51668: F, t51672: F, t51676: F, t51680: F, t51682: F, t62385: F, t62612: F, t40945: F, t40948: F, t40952: F, t40954: F, t40956: F, t40958: F, t51684: F, t51686: F, t51688: F, t51696: F, t51700: F, t51703: F, t51708: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62760, t62763, t62775, t62777, t62788) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3311::<F>(t5977, t860, t231, t2782, t2783, t18657, t233, t689, t869, t10069, t18750, t822);
        let t62792 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3312::<F>(t10657, t14663, t14972, t18714, t2646, t39656, t39731, t4424, t4514, t4526, t51470, t51483, t6017, t6022, t62760, t62763, t62775, t62777, t62788, t820, t837);
        let (t62803, t62825) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3313::<F>(t1568, t4423, t2783, t6041, t786, t2801, t10943, t14546, t18525, t18616, t18681, t2646, t2754, t2815, t40267, t40271, t40273, t40294, t4366, t4494, t4504, t4514, t51505, t51507, t62760, t820, t837);
        let t62856 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3314::<F>(t231, t2782, t2783, t62803, t18689, t2435, t18688, t2439, t2777, t14587, t51548, t10943, t14546, t18525, t18677, t18699, t2646, t40284, t40303, t40314, t40316, t40318, t4504, t4514, t51512, t62760);
        let (t62868, t62887) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3315::<F>(t14602, t14961, t1558, t2482, t4469, t14520, t14568, t14524, t51297, t2801, t4526, t14546, t14547, t18699, t2724, t4366, t4494, t4504, t51519, t51521, t51523, t51527, t51531, t62209);
        let t62912 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3316::<F>(t136, t2457, t39680, t6022, t10073, t18746, t14502, t1559, t18632, t4366, t4504, t51332, t51535, t51537, t51541, t51544, t51546, t51550, t51553, t51560, t62803, t820);
        let t62945 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3317::<F>(t10073, t18742, t10069, t18746, t2718, t6041, t231, t2782, t2783, t62868, t14546, t14547, t18677, t18681, t2646, t2724, t2754, t2811, t4494, t4514, t51436, t51564, t51572, t51576, t51578, t51587, t61866, t820);
        let t62973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3318::<F>(t18729, t2470, t2798, t2723, t2782, t4503, t62760, t2482, t6016, t879, t2801, t14502, t18699, t2754, t40922, t40924, t4424, t4514, t51598, t51600, t51603, t51610, t51614, t51617);
        let (t62983, t62987, t62992, t62999) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3319::<F>(t14563, t14568, t14598, t14600, t4423, t676, t14602, t2482, t2811, t6016, t10535, t136, t2457, t6017);
        let t63002 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3320::<F>(t40938, t4494, t4504, t4514, t51529, t51621, t51623, t51628, t51632, t51635, t51637, t51642, t62641, t62983, t62987, t62992, t62999, t837);
        let t63024 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3321::<F>(t10542, t18726, t14546, t14547, t18677, t4514, t51646, t51653, t51657, t51660, t51668, t51672, t51676, t51680, t51682, t62385, t62612, t820, t837, t879);
        let t63041 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3322::<F>(t40945, t40948, t40952, t40954, t40956, t40958, t4514, t51684, t51686, t51688, t51696, t51700, t51703, t51708, t62868, t837);
    (t62792, t62825, t62856, t62887, t62912, t62945, t62973, t63002, t63024, t63041)
}
