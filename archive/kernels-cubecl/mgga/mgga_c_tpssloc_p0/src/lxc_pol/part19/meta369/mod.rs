//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta369 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1360;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1361;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1362;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1363;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1364;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1365;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1366;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1367;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1368;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1369;
use chunk10::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1370;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta369<F: Float>(t363: F, t42342: F, t42345: F, t43288: F, t3131: F, t3047: F, t3077: F, t10908: F, t3114: F, t1036: F, t10438: F, t221: F, t339: F, t42813: F, t10283: F, t995: F, t10931: F, t135: F, t973: F, t1021: F, t1046: F, t10501: F, t10998: F, t248: F, t2960: F, t3048: F, t350: F, t42348: F, t42759: F, t43273: F, t43277: F, t43281: F, t43285: F, t10216: F, t2978: F, t10479: F, t42333: F, t10922: F, t10489: F, t1041: F, t10868: F, t2776: F, t3061: F, t676: F, t2771: F, t3129: F, t42742: F, t10962: F, t3103: F, t3078: F, t3082: F, t3089: F, t1058: F, t3068: F, t3087: F, t11065: F, t42387: F, t10408: F, t10485: F, t10877: F, t14172: F, t14228: F, t2250: F, t2770: F, t3070: F, t3071: F, t3073: F, t3134: F, t39097: F, t42468: F, t4582: F, t884: F, t974: F, t10250: F, t2970: F, t10195: F, t10231: F, t1005: F, t10375: F, t10475: F, t283: F, t61: F, t10309: F, t10457: F, t10444: F, t354: F, t364: F, t372: F, t10364: F, t10413: F, t10482: F, t10965: F, t10972: F, t3041: F, t3057: F, t3064: F, t3117: F, t3123: F, t41667: F, t41715: F, t977: F, t42337: F, t42409: F, t42459: F, t42499: F, t42540: F, t42580: F, t42621: F, t42723: F, t43099: F, t43141: F, t43181: F, t43223: F, t43267: F, t11018: F, t225: F, t3206: F, t11016: F, t10160: F, t10170: F, t10182: F, t10358: F, t1049: F, t1052: F, t1066: F, t11007: F, t11010: F, t11085: F, t3020: F, t3026: F, t3166: F, t3169: F, t3174: F, t3176: F, t3207: F, t349: F, t388: F, t990: F, t11064: F, t42332: F, t11058: F, t3185: F, t42741: F, t10481: F, t3040: F, t1014: F, t42340: F, t42341: F, t381: F, t23508: F, t360: F, t1003: F, t1022: F, t10359: F, t1060: F, t1063: F, t11027: F, t11031: F, t11043: F, t11066: F, t14590: F, t3180: F, t3186: F, t3188: F, t3189: F, t3196: F, t353: F, t383: F, t4673: F) -> (F, F, F, F, F, F, F, F) {
        let (t43291, t43292, t43298, t43301, t43303, t43307) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1360::<F>(t363, t42342, t42345, t43288, t3131, t3047, t3077, t10908, t3114, t1036, t10438, t221, t339, t42813);
        let t43315 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1361::<F>(t10283, t995, t10931, t135, t973, t1021, t1046, t10501, t10998, t248, t2960, t3048, t350, t42348, t42759, t43273, t43277, t43281, t43285, t43291, t43292, t43298, t43301, t43303, t43307);
        let (t43317, t43322, t43325, t43332, t43336) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1362::<F>(t10216, t2978, t10479, t42333, t10922, t2960, t10489, t3048, t1041, t10868, t248, t2776);
        let (t43341, t43343, t43350, t43352, t43354) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1363::<F>(t3061, t676, t1041, t248, t2771, t3129, t42742, t10962, t3103, t3078, t3082, t3089);
        let t43366 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1364::<F>(t1058, t3068, t3087, t363, t11065, t42387, t10408, t1041, t10485, t10877, t14172, t14228, t2250, t2770, t3070, t3071, t3073, t3134, t39097, t42468, t43317, t43322, t43325, t43332, t43336, t43341, t43343, t43350, t43352, t43354, t4582, t884, t973, t974);
        let (t43374, t43377, t43382, t43385, t43398) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1365::<F>(t10250, t2970, t973, t10195, t10231, t1005, t10375, t10475, t42342, t42345, t2770, t283);
        let t43415 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1366::<F>(t43398, t61, t10309, t1041, t10457, t248, t10444, t354, t364, t372, t1021, t10364, t10408, t10413, t1046, t10482, t10962, t10965, t10972, t2771, t2960, t3041, t3057, t3064, t3117, t3123, t41667, t41715, t42348, t43374, t43377, t43382, t43385, t973, t977);
        let t43419 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1367::<F>(t42337, t42409, t42459, t42499, t42540, t42580, t42621, t42723, t43099, t43141, t43181, t43223, t43267, t43315, t43366, t43415);
        let t43447 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1368::<F>(t11018, t225, t3206, t11016, t10160, t10170, t10182, t10358, t1049, t1052, t1066, t11007, t11010, t11085, t3020, t3026, t3166, t3169, t3174, t3176, t3207, t349, t388, t43419, t990);
        let (t43470, t43473, t43480, t43483, t43489, t43503) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1369::<F>(t11064, t42332, t11058, t3185, t42741, t10481, t1049, t3040, t3166, t1014, t42340, t42341);
        let (t43504, t43512) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1370::<F>(t381, t42348, t23508, t360, t1003, t1022, t10359, t1058, t1060, t1063, t11007, t11027, t11031, t11043, t11065, t11066, t14590, t3180, t3186, t3188, t3189, t3196, t353, t383, t43419, t43480, t43483, t43489, t43503, t4673);
    (t43292, t43447, t43470, t43473, t43483, t43489, t43504, t43512)
}
