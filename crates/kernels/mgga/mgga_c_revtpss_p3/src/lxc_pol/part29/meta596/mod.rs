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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta596<F: Float>(t26544: F, t27216: F, t26506: F, t27213: F, t28399: F, t686: F, t72: F, t7058: F, t103000: F, t93371: F, t25410: F, t8011: F, t93240: F, t7064: F, t14662: F, t2061: F, t231: F, t25391: F, t26515: F, t26550: F, t27199: F, t27353: F, t2771: F, t51525: F, t7070: F, t7076: F, t8006: F, t93118: F, t95762: F, t95766: F, t95768: F, t99289: F, t28447: F, t689: F, t887: F, t26485: F, t99463: F, t102986: F, t25387: F, t1580: F, t2439: F, t26434: F, t2453: F, t2458: F, t7998: F, t4423: F, t7398: F, t7420: F, t95774: F, t95779: F, t95783: F, t95786: F, t95790: F, t95794: F, t95796: F, t95798: F, t99303: F, t41040: F, t685: F, t28313: F, t93317: F, t4534: F, t7384: F, t14489: F, t14495: F, t1558: F, t25383: F, t26473: F, t26547: F, t27275: F, t28310: F, t28378: F, t28425: F, t4487: F, t51529: F, t51574: F, t51608: F, t7403: F, t7424: F, t95825: F, t99316: F, t99512: F, t213: F, t28340: F, t14983: F, t26497: F, t14485: F, t4481: F, t95743: F, t10073: F, t25402: F, t7056: F, t7997: F, t26519: F, t98867: F, t1579: F, t2828: F, t2829: F, t28394: F, t7071: F, t95807: F, t95808: F, t95811: F, t95813: F, t95823: F, t98937: F, t98949: F, t92952: F, t92956: F, t98940: F, t98943: F, t98945: F, t98947: F, t98951: F, t98953: F, t98955: F, t98957: F, t98964: F, t98968: F, t98972: F, t98976: F, t98979: F, t92963: F, t92966: F, t92969: F, t92971: F, t92979: F, t95666: F, t98970: F, t98983: F, t98991: F, t99000: F, t99006: F, t92991: F, t95671: F, t98985: F, t98989: F, t98993: F, t98995: F, t98997: F, t99002: F, t99009: F, t99011: F, t99013: F, t99019: F, t99021: F, t99023: F, t99026: F, t99029: F, t99033: F, t99035: F, t99015: F, t99017: F, t99031: F, t99041: F, t99044: F, t99050: F, t93001: F, t95673: F, t95674: F, t95675: F, t99046: F, t99048: F, t99052: F, t99054: F, t99056: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t103103, t103114, t103117, t103119, t103122, t103130) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2003::<F>(t26544, t27216, t26506, t27213, t28399, t686, t72, t7058, t103000, t93371, t25410, t8011, t93240);
        let t103137 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2004::<F>(t103117, t7064, t103103, t103114, t103119, t103122, t103130, t14662, t2061, t231, t25391, t26515, t26550, t27199, t27353, t2771, t51525, t7070, t7076, t8006, t93118, t95762, t95766, t95768, t99289);
        let (t103140, t103142, t103156, t103158, t103161) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2005::<F>(t28447, t689, t887, t26485, t99463, t102986, t25387, t1580, t2439, t26434, t2453, t2458, t7998);
        let t103166 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2006::<F>(t103140, t103142, t103156, t103158, t103161, t231, t4423, t7070, t7076, t7398, t7420, t95774, t95779, t95783, t95786, t95790, t95794, t95796, t95798, t99303);
        let (t103182, t103210) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2007::<F>(t41040, t685, t28313, t93317, t4534, t689, t7384, t14489, t14495, t1558, t231, t25383, t25391, t26473, t26547, t26550, t27275, t27353, t28310, t28378, t28425, t4487, t51529, t51574, t51608, t7070, t7076, t7403, t7424, t95825, t99316, t99512);
        let (t103212, t103216, t103219, t103220, t103224, t103234) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2008::<F>(t213, t28340, t26544, t27213, t14983, t26497, t14485, t4481, t95743, t10073, t25402, t7056, t7997);
        let t103242 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2009::<F>(t26519, t98867, t103212, t103216, t103219, t103220, t103224, t103234, t1579, t26473, t2828, t2829, t28394, t7070, t7071, t7997, t887, t95807, t95808, t95811, t95813, t95823);
        let t103259 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2010::<F>(t98937, t98949, t92952, t92956, t98940, t98943, t98945, t98947, t98951, t98953, t98955, t98957);
        let t103271 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2011::<F>(t98964, t98968, t98972, t98976, t98979, t92963, t92966, t92969, t92971, t92979, t95666, t98970);
        let t103284 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2012::<F>(t98983, t98991, t99000, t99006, t92991, t95671, t98985, t98989, t98993, t98995, t98997, t99002);
        let t103298 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2013::<F>(t99009, t99011, t99013, t99019, t99021, t99023, t99026, t99029, t99033, t99035, t99015, t99017, t99031);
        let t103310 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2014::<F>(t99041, t99044, t99050, t93001, t95673, t95674, t95675, t99046, t99048, t99052, t99054, t99056);
    (t103137, t103166, t103182, t103210, t103242, t103259, t103271, t103284, t103298, t103310)
}
