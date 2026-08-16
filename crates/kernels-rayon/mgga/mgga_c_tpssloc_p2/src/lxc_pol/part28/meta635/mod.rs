//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta635 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2011;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2012;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2013;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2014;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2015;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2016;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2017;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2018;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta635(t90642: f64, t90645: f64, t90659: f64, t90663: f64, t90686: f64, t90701: f64, t12021: f64, t12033: f64, t1375: f64, t16460: f64, t16475: f64, t2092: f64, t27062: f64, t27115: f64, t3758: f64, t3882: f64, t3887: f64, t3888: f64, t3911: f64, t55134: f64, t7194: f64, t7199: f64, t7925: f64, t7936: f64, t81264: f64, t81267: f64, t84423: f64, t90639: f64, t90690: f64, t90704: f64, t90707: f64, t90749: f64, t90759: f64, t90781: f64, t90789: f64, t90791: f64, t90794: f64, t90797: f64, t12240: f64, t16033: f64, t27074: f64, t27078: f64, t5334: f64, t90747: f64, t90752: f64, t90757: f64, t90763: f64, t90770: f64, t90774: f64, t90778: f64, t90785: f64, t90805: f64, t2085: f64, t5286: f64, t1824: f64, t7191: f64, t90837: f64, t1352: f64, t16123: f64, t2089: f64, t3851: f64, t5250: f64, t5344: f64, t90801: f64, t90807: f64, t90812: f64, t90816: f64, t90821: f64, t90825: f64, t90829: f64, t90832: f64, t90835: f64, t90840: f64, t90844: f64, t90859: f64, t90864: f64, t90866: f64, t90868: f64, t1332: f64, t1336: f64, t16047: f64, t16048: f64, t16055: f64, t24117: f64, t24131: f64, t27075: f64, t27097: f64, t27105: f64, t3793: f64, t3856: f64, t5234: f64, t81022: f64, t90848: f64, t90852: f64, t90856: f64, t90873: f64, t90898: f64, t90900: f64, t16206: f64, t27098: f64, t3777: f64, t7208: f64, t81037: f64, t81039: f64, t81041: f64, t81043: f64, t81047: f64, t81050: f64, t81061: f64, t81066: f64, t90883: f64, t90887: f64, t90892: f64, t90895: f64, t90912: f64, t24103: f64, t3773: f64, t7934: f64, t81069: f64, t81076: f64, t81080: f64, t81083: f64, t81099: f64, t84480: f64, t84481: f64, t90907: f64, t90910: f64, t90917: f64, t90921: f64, t90929: f64, t90933: f64, t90956: f64, t90961: f64, t90963: f64, t90970: f64, t90980: f64, t90983: f64, t90987: f64, t90993: f64, t1338: f64, t27051: f64, t12267: f64, t1825: f64, t24128: f64, t27103: f64, t7932: f64, t81115: f64, t81125: f64, t84581: f64, t90968: f64, t91000: f64, t91010: f64, t91113: f64, t91120: f64, t91094: f64, t91096: f64, t91098: f64, t91101: f64, t91103: f64, t91105: f64, t91107: f64, t91109: f64, t91116: f64, t91118: f64, t91122: f64, t91124: f64, t91126: f64, t91128: f64, t91130: f64, t91135: f64, t91137: f64, t91140: f64, t91149: f64, t91154: f64, t91158: f64, t91161: f64, t91167: f64, t91170: f64, t91133: f64, t91143: f64, t91145: f64, t91147: f64, t91163: f64, t91165: f64, t91173: f64, t91176: f64, t91179: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t93465 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2011(t90642, t90645, t90659, t90663, t90686, t90701, t12021, t12033, t1375, t16460, t16475, t2092, t27062, t27115, t3758, t3882, t3887, t3888, t3911, t55134, t7194, t7199, t7925, t7936, t81264, t81267, t84423, t90639, t90690, t90704);
        let (t93467, t93492) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2012(t90707, t90749, t90759, t90781, t90789, t90791, t90794, t90797, t12240, t16033, t27074, t27078, t5334, t90747, t90752, t90757, t90763, t90770, t90774, t90778, t90785);
        let (t93505, t93519) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2013(t90805, t2085, t5286, t1824, t7191, t90837, t1352, t16123, t2089, t27074, t3851, t5250, t5334, t5344, t90801, t90807, t90812, t90816, t90821, t90825, t90829, t90832, t90835, t90840);
        let t93546 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2014(t90844, t90859, t90864, t90866, t90868, t1332, t1336, t16047, t16048, t16055, t24117, t24131, t27074, t27075, t27097, t27105, t3793, t3856, t5234, t5334, t81022, t90848, t90852, t90856, t90873);
        let t93567 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2015(t90898, t90900, t1336, t16206, t27097, t27098, t3777, t3851, t7208, t81037, t81039, t81041, t81043, t81047, t81050, t81061, t81066, t90883, t90887, t90892, t90895);
        let t93587 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2016(t90912, t1352, t24103, t3773, t5234, t5344, t7934, t81069, t81076, t81080, t81083, t81099, t84480, t84481, t90907, t90910, t90917, t90921, t90929, t90933, t93505);
        let t93612 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2017(t90956, t90961, t90963, t90970, t90980, t90983, t90987, t90993, t1338, t27051, t12267, t1336, t1352, t1825, t24128, t27074, t27103, t3777, t3856, t5234, t5344, t7932, t81115, t81125, t84581, t90968);
        let (t93615, t93618, t93642) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2018(t91000, t91010, t91113, t91120, t91094, t91096, t91098, t91101, t91103, t91105, t91107, t91109, t91116, t91118, t91122, t91124, t91126, t91128, t91130);
        let t93661 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2019(t91135, t91137, t91140, t91149, t91154, t91158, t91161, t91167, t91170, t91133, t91143, t91145, t91147, t91163, t91165, t91173, t91176, t91179);
    (t93465, t93467, t93492, t93519, t93546, t93567, t93587, t93612, t93615, t93618, t93642, t93661)
}
