//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta596 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2003;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2004;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2005;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2006;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2007;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2008;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2009;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2010;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2011;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2012;
use chunk10::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2013;
use chunk11::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta596(t26544: f64, t27216: f64, t26506: f64, t27213: f64, t28399: f64, t686: f64, t72: f64, t7058: f64, t103000: f64, t93371: f64, t25410: f64, t8011: f64, t93240: f64, t7064: f64, t14662: f64, t2061: f64, t231: f64, t25391: f64, t26515: f64, t26550: f64, t27199: f64, t27353: f64, t2771: f64, t51525: f64, t7070: f64, t7076: f64, t8006: f64, t93118: f64, t95762: f64, t95766: f64, t95768: f64, t99289: f64, t28447: f64, t689: f64, t887: f64, t26485: f64, t99463: f64, t102986: f64, t25387: f64, t1580: f64, t2439: f64, t26434: f64, t2453: f64, t2458: f64, t7998: f64, t4423: f64, t7398: f64, t7420: f64, t95774: f64, t95779: f64, t95783: f64, t95786: f64, t95790: f64, t95794: f64, t95796: f64, t95798: f64, t99303: f64, t41040: f64, t685: f64, t28313: f64, t93317: f64, t4534: f64, t7384: f64, t14489: f64, t14495: f64, t1558: f64, t25383: f64, t26473: f64, t26547: f64, t27275: f64, t28310: f64, t28378: f64, t28425: f64, t4487: f64, t51529: f64, t51574: f64, t51608: f64, t7403: f64, t7424: f64, t95825: f64, t99316: f64, t99512: f64, t213: f64, t28340: f64, t14983: f64, t26497: f64, t14485: f64, t4481: f64, t95743: f64, t10073: f64, t25402: f64, t7056: f64, t7997: f64, t26519: f64, t98867: f64, t1579: f64, t2828: f64, t2829: f64, t28394: f64, t7071: f64, t95807: f64, t95808: f64, t95811: f64, t95813: f64, t95823: f64, t98937: f64, t98949: f64, t92952: f64, t92956: f64, t98940: f64, t98943: f64, t98945: f64, t98947: f64, t98951: f64, t98953: f64, t98955: f64, t98957: f64, t98964: f64, t98968: f64, t98972: f64, t98976: f64, t98979: f64, t92963: f64, t92966: f64, t92969: f64, t92971: f64, t92979: f64, t95666: f64, t98970: f64, t98983: f64, t98991: f64, t99000: f64, t99006: f64, t92991: f64, t95671: f64, t98985: f64, t98989: f64, t98993: f64, t98995: f64, t98997: f64, t99002: f64, t99009: f64, t99011: f64, t99013: f64, t99019: f64, t99021: f64, t99023: f64, t99026: f64, t99029: f64, t99033: f64, t99035: f64, t99015: f64, t99017: f64, t99031: f64, t99041: f64, t99044: f64, t99050: f64, t93001: f64, t95673: f64, t95674: f64, t95675: f64, t99046: f64, t99048: f64, t99052: f64, t99054: f64, t99056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t103103, t103114, t103117, t103119, t103122, t103130) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2003(t26544, t27216, t26506, t27213, t28399, t686, t72, t7058, t103000, t93371, t25410, t8011, t93240);
        let t103137 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2004(t103117, t7064, t103103, t103114, t103119, t103122, t103130, t14662, t2061, t231, t25391, t26515, t26550, t27199, t27353, t2771, t51525, t7070, t7076, t8006, t93118, t95762, t95766, t95768, t99289);
        let (t103140, t103142, t103156, t103158, t103161) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2005(t28447, t689, t887, t26485, t99463, t102986, t25387, t1580, t2439, t26434, t2453, t2458, t7998);
        let t103166 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2006(t103140, t103142, t103156, t103158, t103161, t231, t4423, t7070, t7076, t7398, t7420, t95774, t95779, t95783, t95786, t95790, t95794, t95796, t95798, t99303);
        let (t103182, t103210) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2007(t41040, t685, t28313, t93317, t4534, t689, t7384, t14489, t14495, t1558, t231, t25383, t25391, t26473, t26547, t26550, t27275, t27353, t28310, t28378, t28425, t4487, t51529, t51574, t51608, t7070, t7076, t7403, t7424, t95825, t99316, t99512);
        let (t103212, t103216, t103219, t103220, t103224, t103234) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2008(t213, t28340, t26544, t27213, t14983, t26497, t14485, t4481, t95743, t10073, t25402, t7056, t7997);
        let t103242 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2009(t26519, t98867, t103212, t103216, t103219, t103220, t103224, t103234, t1579, t26473, t2828, t2829, t28394, t7070, t7071, t7997, t887, t95807, t95808, t95811, t95813, t95823);
        let t103259 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2010(t98937, t98949, t92952, t92956, t98940, t98943, t98945, t98947, t98951, t98953, t98955, t98957);
        let t103271 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2011(t98964, t98968, t98972, t98976, t98979, t92963, t92966, t92969, t92971, t92979, t95666, t98970);
        let t103284 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2012(t98983, t98991, t99000, t99006, t92991, t95671, t98985, t98989, t98993, t98995, t98997, t99002);
        let t103298 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2013(t99009, t99011, t99013, t99019, t99021, t99023, t99026, t99029, t99033, t99035, t99015, t99017, t99031);
        let t103310 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2014(t99041, t99044, t99050, t93001, t95673, t95674, t95675, t99046, t99048, t99052, t99054, t99056);
    (t103137, t103166, t103182, t103210, t103242, t103259, t103271, t103284, t103298, t103310)
}
