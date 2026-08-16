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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta635<F: Float>(t90642: F, t90645: F, t90659: F, t90663: F, t90686: F, t90701: F, t12021: F, t12033: F, t1375: F, t16460: F, t16475: F, t2092: F, t27062: F, t27115: F, t3758: F, t3882: F, t3887: F, t3888: F, t3911: F, t55134: F, t7194: F, t7199: F, t7925: F, t7936: F, t81264: F, t81267: F, t84423: F, t90639: F, t90690: F, t90704: F, t90707: F, t90749: F, t90759: F, t90781: F, t90789: F, t90791: F, t90794: F, t90797: F, t12240: F, t16033: F, t27074: F, t27078: F, t5334: F, t90747: F, t90752: F, t90757: F, t90763: F, t90770: F, t90774: F, t90778: F, t90785: F, t90805: F, t2085: F, t5286: F, t1824: F, t7191: F, t90837: F, t1352: F, t16123: F, t2089: F, t3851: F, t5250: F, t5344: F, t90801: F, t90807: F, t90812: F, t90816: F, t90821: F, t90825: F, t90829: F, t90832: F, t90835: F, t90840: F, t90844: F, t90859: F, t90864: F, t90866: F, t90868: F, t1332: F, t1336: F, t16047: F, t16048: F, t16055: F, t24117: F, t24131: F, t27075: F, t27097: F, t27105: F, t3793: F, t3856: F, t5234: F, t81022: F, t90848: F, t90852: F, t90856: F, t90873: F, t90898: F, t90900: F, t16206: F, t27098: F, t3777: F, t7208: F, t81037: F, t81039: F, t81041: F, t81043: F, t81047: F, t81050: F, t81061: F, t81066: F, t90883: F, t90887: F, t90892: F, t90895: F, t90912: F, t24103: F, t3773: F, t7934: F, t81069: F, t81076: F, t81080: F, t81083: F, t81099: F, t84480: F, t84481: F, t90907: F, t90910: F, t90917: F, t90921: F, t90929: F, t90933: F, t90956: F, t90961: F, t90963: F, t90970: F, t90980: F, t90983: F, t90987: F, t90993: F, t1338: F, t27051: F, t12267: F, t1825: F, t24128: F, t27103: F, t7932: F, t81115: F, t81125: F, t84581: F, t90968: F, t91000: F, t91010: F, t91113: F, t91120: F, t91094: F, t91096: F, t91098: F, t91101: F, t91103: F, t91105: F, t91107: F, t91109: F, t91116: F, t91118: F, t91122: F, t91124: F, t91126: F, t91128: F, t91130: F, t91135: F, t91137: F, t91140: F, t91149: F, t91154: F, t91158: F, t91161: F, t91167: F, t91170: F, t91133: F, t91143: F, t91145: F, t91147: F, t91163: F, t91165: F, t91173: F, t91176: F, t91179: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t93465 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2011::<F>(t90642, t90645, t90659, t90663, t90686, t90701, t12021, t12033, t1375, t16460, t16475, t2092, t27062, t27115, t3758, t3882, t3887, t3888, t3911, t55134, t7194, t7199, t7925, t7936, t81264, t81267, t84423, t90639, t90690, t90704);
        let (t93467, t93492) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2012::<F>(t90707, t90749, t90759, t90781, t90789, t90791, t90794, t90797, t12240, t16033, t27074, t27078, t5334, t90747, t90752, t90757, t90763, t90770, t90774, t90778, t90785);
        let (t93505, t93519) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2013::<F>(t90805, t2085, t5286, t1824, t7191, t90837, t1352, t16123, t2089, t27074, t3851, t5250, t5334, t5344, t90801, t90807, t90812, t90816, t90821, t90825, t90829, t90832, t90835, t90840);
        let t93546 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2014::<F>(t90844, t90859, t90864, t90866, t90868, t1332, t1336, t16047, t16048, t16055, t24117, t24131, t27074, t27075, t27097, t27105, t3793, t3856, t5234, t5334, t81022, t90848, t90852, t90856, t90873);
        let t93567 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2015::<F>(t90898, t90900, t1336, t16206, t27097, t27098, t3777, t3851, t7208, t81037, t81039, t81041, t81043, t81047, t81050, t81061, t81066, t90883, t90887, t90892, t90895);
        let t93587 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2016::<F>(t90912, t1352, t24103, t3773, t5234, t5344, t7934, t81069, t81076, t81080, t81083, t81099, t84480, t84481, t90907, t90910, t90917, t90921, t90929, t90933, t93505);
        let t93612 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2017::<F>(t90956, t90961, t90963, t90970, t90980, t90983, t90987, t90993, t1338, t27051, t12267, t1336, t1352, t1825, t24128, t27074, t27103, t3777, t3856, t5234, t5344, t7932, t81115, t81125, t84581, t90968);
        let (t93615, t93618, t93642) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2018::<F>(t91000, t91010, t91113, t91120, t91094, t91096, t91098, t91101, t91103, t91105, t91107, t91109, t91116, t91118, t91122, t91124, t91126, t91128, t91130);
        let t93661 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2019::<F>(t91135, t91137, t91140, t91149, t91154, t91158, t91161, t91167, t91170, t91133, t91143, t91145, t91147, t91163, t91165, t91173, t91176, t91179);
    (t93465, t93467, t93492, t93519, t93546, t93567, t93587, t93612, t93615, t93618, t93642, t93661)
}
