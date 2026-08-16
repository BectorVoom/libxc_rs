//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta384 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1402;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1403;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1404;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1405;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1406;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1407;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1408;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1409;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1410;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1411;
use chunk10::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1412;
use chunk11::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta384<F: Float>(t10504: F, t138: F, t2438: F, t2828: F, t11044: F, t11050: F, t11015: F, t2461: F, t11010: F, t689: F, t779: F, t2769: F, t786: F, t861: F, t10997: F, t11007: F, t252: F, t11009: F, t123: F, t676: F, t41026: F, t41029: F, t41032: F, t41034: F, t41037: F, t41038: F, t41043: F, t41049: F, t41052: F, t11006: F, t256: F, t225: F, t2771: F, t2782: F, t886: F, t2441: F, t39515: F, t9302: F, t2465: F, t9291: F, t10982: F, t860: F, t9646: F, t2434: F, t10115: F, t251: F, t887: F, t2439: F, t2440: F, t2829: F, t10977: F, t686: F, t72: F, t10513: F, t10978: F, t213: F, t257: F, t2765: F, t2770: F, t2772: F, t40876: F, t865: F, t11061: F, t11064: F, t10489: F, t198: F, t207: F, t2403: F, t2404: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39738: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t40975: F, t41023: F, t775: F, t892: F, t11075: F, t1940: F, t2394: F, t2408: F, t2832: F, t39760: F, t39764: F, t39767: F, t39770: F, t39773: F, t39775: F, t39778: F, t39780: F, t39783: F, t39786: F, t39791: F, t39795: F, t4541: F, t2410: F, t10818: F, t11071: F, t2393: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t39857: F, t39859: F, t39861: F, t40084: F, t40088: F, t40240: F, t2430: F, t40093: F, t40095: F, t40099: F, t40103: F, t40106: F, t40109: F, t40111: F, t40115: F, t40117: F, t40120: F, t40122: F, t40126: F, t11084: F, t14375: F, t262: F, t39989: F, t40128: F, t40131: F, t40133: F, t40137: F, t40140: F, t40142: F, t40144: F, t40146: F, t40149: F, t40151: F, t10627: F, t40067: F, t40072: F, t40155: F, t40157: F, t40160: F, t40163: F, t40167: F, t40171: F, t40173: F, t40175: F, t40179: F, t40184: F, t40187: F, t890: F, t11054: F, t2411: F, t40076: F, t40079: F, t40190: F, t40194: F, t40198: F, t40202: F, t40204: F, t40206: F, t40209: F, t40212: F) -> (F, F, F, F, F, F, F) {
        let (t41056, t41058, t41060, t41063, t41066) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1402::<F>(t10504, t138, t2438, t2828, t11044, t11050, t11015, t2461, t11010, t689, t779, t2769, t786, t861);
        let t41075 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1403::<F>(t10997, t41066, t11007, t252, t786, t11009, t123, t676, t41026, t41029, t41032, t41034, t41037, t41038, t41043, t41049, t41052, t41056, t41058, t41060, t41063);
        let (t41078, t41079, t41085, t41092, t41095, t41098) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1404::<F>(t11006, t256, t225, t2771, t2828, t252, t2769, t2782, t886, t2441, t39515, t10504, t138, t9302);
        let (t41102, t41105, t41115, t41118) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1405::<F>(t123, t2465, t886, t9291, t10982, t860, t9646, t2434, t2828, t10115, t251, t887);
        let t41131 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1406::<F>(t2439, t2440, t2829, t10977, t2465, t686, t72, t10513, t10978, t11010, t213, t225, t257, t2765, t2770, t2772, t40876, t41078, t41079, t41085, t41092, t41095, t41098, t41102, t41105, t41115, t41118, t865);
        let t41141 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1407::<F>(t11061, t11064, t10489, t198, t207, t2403, t2404, t39528, t39531, t39534, t39537, t39540, t39738, t39741, t39744, t39747, t39750, t39756, t40975, t41023, t41075, t41131, t775, t892);
        let t41150 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1408::<F>(t11064, t11075, t1940, t2394, t2408, t2832, t39760, t39764, t39767, t39770, t39773, t39775, t39778, t39780, t39783, t39786, t39791, t39795, t4541);
        let t41168 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1409::<F>(t2408, t2410, t2832, t775, t10818, t11071, t198, t207, t2393, t2403, t2404, t39799, t39807, t39813, t39818, t39823, t39857, t39859, t39861, t40084, t40088, t40240, t4541);
        let t41174 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1410::<F>(t11075, t2403, t2430, t40093, t40095, t40099, t40103, t40106, t40109, t40111, t40115, t40117, t40120, t40122, t40126);
        let t41185 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1411::<F>(t10489, t11084, t14375, t198, t2403, t2430, t262, t39989, t40128, t40131, t40133, t40137, t40140, t40142, t40144, t40146, t40149, t40151, t4541, t775);
        let t41191 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1412::<F>(t10627, t198, t40067, t40072, t40155, t40157, t40160, t40163, t40167, t40171, t40173, t40175, t40179, t40184, t40187, t890, t892);
        let t41208 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1413::<F>(t2832, t11054, t892, t11084, t1940, t198, t207, t2394, t2403, t2411, t40076, t40079, t40190, t40194, t40198, t40202, t40204, t40206, t40209, t40212, t4541, t775, t890);
    (t41141, t41150, t41168, t41174, t41185, t41191, t41208)
}
