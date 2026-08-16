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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta384(t10504: f64, t138: f64, t2438: f64, t2828: f64, t11044: f64, t11050: f64, t11015: f64, t2461: f64, t11010: f64, t689: f64, t779: f64, t2769: f64, t786: f64, t861: f64, t10997: f64, t11007: f64, t252: f64, t11009: f64, t123: f64, t676: f64, t41026: f64, t41029: f64, t41032: f64, t41034: f64, t41037: f64, t41038: f64, t41043: f64, t41049: f64, t41052: f64, t11006: f64, t256: f64, t225: f64, t2771: f64, t2782: f64, t886: f64, t2441: f64, t39515: f64, t9302: f64, t2465: f64, t9291: f64, t10982: f64, t860: f64, t9646: f64, t2434: f64, t10115: f64, t251: f64, t887: f64, t2439: f64, t2440: f64, t2829: f64, t10977: f64, t686: f64, t72: f64, t10513: f64, t10978: f64, t213: f64, t257: f64, t2765: f64, t2770: f64, t2772: f64, t40876: f64, t865: f64, t11061: f64, t11064: f64, t10489: f64, t198: f64, t207: f64, t2403: f64, t2404: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t39738: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t40975: f64, t41023: f64, t775: f64, t892: f64, t11075: f64, t1940: f64, t2394: f64, t2408: f64, t2832: f64, t39760: f64, t39764: f64, t39767: f64, t39770: f64, t39773: f64, t39775: f64, t39778: f64, t39780: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t4541: f64, t2410: f64, t10818: f64, t11071: f64, t2393: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t39857: f64, t39859: f64, t39861: f64, t40084: f64, t40088: f64, t40240: f64, t2430: f64, t40093: f64, t40095: f64, t40099: f64, t40103: f64, t40106: f64, t40109: f64, t40111: f64, t40115: f64, t40117: f64, t40120: f64, t40122: f64, t40126: f64, t11084: f64, t14375: f64, t262: f64, t39989: f64, t40128: f64, t40131: f64, t40133: f64, t40137: f64, t40140: f64, t40142: f64, t40144: f64, t40146: f64, t40149: f64, t40151: f64, t10627: f64, t40067: f64, t40072: f64, t40155: f64, t40157: f64, t40160: f64, t40163: f64, t40167: f64, t40171: f64, t40173: f64, t40175: f64, t40179: f64, t40184: f64, t40187: f64, t890: f64, t11054: f64, t2411: f64, t40076: f64, t40079: f64, t40190: f64, t40194: f64, t40198: f64, t40202: f64, t40204: f64, t40206: f64, t40209: f64, t40212: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t41056, t41058, t41060, t41063, t41066) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1402(t10504, t138, t2438, t2828, t11044, t11050, t11015, t2461, t11010, t689, t779, t2769, t786, t861);
        let t41075 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1403(t10997, t41066, t11007, t252, t786, t11009, t123, t676, t41026, t41029, t41032, t41034, t41037, t41038, t41043, t41049, t41052, t41056, t41058, t41060, t41063);
        let (t41078, t41079, t41085, t41092, t41095, t41098) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1404(t11006, t256, t225, t2771, t2828, t252, t2769, t2782, t886, t2441, t39515, t10504, t138, t9302);
        let (t41102, t41105, t41115, t41118) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1405(t123, t2465, t886, t9291, t10982, t860, t9646, t2434, t2828, t10115, t251, t887);
        let t41131 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1406(t2439, t2440, t2829, t10977, t2465, t686, t72, t10513, t10978, t11010, t213, t225, t257, t2765, t2770, t2772, t40876, t41078, t41079, t41085, t41092, t41095, t41098, t41102, t41105, t41115, t41118, t865);
        let t41141 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1407(t11061, t11064, t10489, t198, t207, t2403, t2404, t39528, t39531, t39534, t39537, t39540, t39738, t39741, t39744, t39747, t39750, t39756, t40975, t41023, t41075, t41131, t775, t892);
        let t41150 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1408(t11064, t11075, t1940, t2394, t2408, t2832, t39760, t39764, t39767, t39770, t39773, t39775, t39778, t39780, t39783, t39786, t39791, t39795, t4541);
        let t41168 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1409(t2408, t2410, t2832, t775, t10818, t11071, t198, t207, t2393, t2403, t2404, t39799, t39807, t39813, t39818, t39823, t39857, t39859, t39861, t40084, t40088, t40240, t4541);
        let t41174 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1410(t11075, t2403, t2430, t40093, t40095, t40099, t40103, t40106, t40109, t40111, t40115, t40117, t40120, t40122, t40126);
        let t41185 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1411(t10489, t11084, t14375, t198, t2403, t2430, t262, t39989, t40128, t40131, t40133, t40137, t40140, t40142, t40144, t40146, t40149, t40151, t4541, t775);
        let t41191 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1412(t10627, t198, t40067, t40072, t40155, t40157, t40160, t40163, t40167, t40171, t40173, t40175, t40179, t40184, t40187, t890, t892);
        let t41208 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1413(t2832, t11054, t892, t11084, t1940, t198, t207, t2394, t2403, t2411, t40076, t40079, t40190, t40194, t40198, t40202, t40204, t40206, t40209, t40212, t4541, t775, t890);
    (t41141, t41150, t41168, t41174, t41185, t41191, t41208)
}
