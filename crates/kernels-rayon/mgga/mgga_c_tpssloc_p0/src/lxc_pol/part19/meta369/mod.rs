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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta369(t363: f64, t42342: f64, t42345: f64, t43288: f64, t3131: f64, t3047: f64, t3077: f64, t10908: f64, t3114: f64, t1036: f64, t10438: f64, t221: f64, t339: f64, t42813: f64, t10283: f64, t995: f64, t10931: f64, t135: f64, t973: f64, t1021: f64, t1046: f64, t10501: f64, t10998: f64, t248: f64, t2960: f64, t3048: f64, t350: f64, t42348: f64, t42759: f64, t43273: f64, t43277: f64, t43281: f64, t43285: f64, t10216: f64, t2978: f64, t10479: f64, t42333: f64, t10922: f64, t10489: f64, t1041: f64, t10868: f64, t2776: f64, t3061: f64, t676: f64, t2771: f64, t3129: f64, t42742: f64, t10962: f64, t3103: f64, t3078: f64, t3082: f64, t3089: f64, t1058: f64, t3068: f64, t3087: f64, t11065: f64, t42387: f64, t10408: f64, t10485: f64, t10877: f64, t14172: f64, t14228: f64, t2250: f64, t2770: f64, t3070: f64, t3071: f64, t3073: f64, t3134: f64, t39097: f64, t42468: f64, t4582: f64, t884: f64, t974: f64, t10250: f64, t2970: f64, t10195: f64, t10231: f64, t1005: f64, t10375: f64, t10475: f64, t283: f64, t61: f64, t10309: f64, t10457: f64, t10444: f64, t354: f64, t364: f64, t372: f64, t10364: f64, t10413: f64, t10482: f64, t10965: f64, t10972: f64, t3041: f64, t3057: f64, t3064: f64, t3117: f64, t3123: f64, t41667: f64, t41715: f64, t977: f64, t42337: f64, t42409: f64, t42459: f64, t42499: f64, t42540: f64, t42580: f64, t42621: f64, t42723: f64, t43099: f64, t43141: f64, t43181: f64, t43223: f64, t43267: f64, t11018: f64, t225: f64, t3206: f64, t11016: f64, t10160: f64, t10170: f64, t10182: f64, t10358: f64, t1049: f64, t1052: f64, t1066: f64, t11007: f64, t11010: f64, t11085: f64, t3020: f64, t3026: f64, t3166: f64, t3169: f64, t3174: f64, t3176: f64, t3207: f64, t349: f64, t388: f64, t990: f64, t11064: f64, t42332: f64, t11058: f64, t3185: f64, t42741: f64, t10481: f64, t3040: f64, t1014: f64, t42340: f64, t42341: f64, t381: f64, t23508: f64, t360: f64, t1003: f64, t1022: f64, t10359: f64, t1060: f64, t1063: f64, t11027: f64, t11031: f64, t11043: f64, t11066: f64, t14590: f64, t3180: f64, t3186: f64, t3188: f64, t3189: f64, t3196: f64, t353: f64, t383: f64, t4673: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43291, t43292, t43298, t43301, t43303, t43307) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1360(t363, t42342, t42345, t43288, t3131, t3047, t3077, t10908, t3114, t1036, t10438, t221, t339, t42813);
        let t43315 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1361(t10283, t995, t10931, t135, t973, t1021, t1046, t10501, t10998, t248, t2960, t3048, t350, t42348, t42759, t43273, t43277, t43281, t43285, t43291, t43292, t43298, t43301, t43303, t43307);
        let (t43317, t43322, t43325, t43332, t43336) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1362(t10216, t2978, t10479, t42333, t10922, t2960, t10489, t3048, t1041, t10868, t248, t2776);
        let (t43341, t43343, t43350, t43352, t43354) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1363(t3061, t676, t1041, t248, t2771, t3129, t42742, t10962, t3103, t3078, t3082, t3089);
        let t43366 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1364(t1058, t3068, t3087, t363, t11065, t42387, t10408, t1041, t10485, t10877, t14172, t14228, t2250, t2770, t3070, t3071, t3073, t3134, t39097, t42468, t43317, t43322, t43325, t43332, t43336, t43341, t43343, t43350, t43352, t43354, t4582, t884, t973, t974);
        let (t43374, t43377, t43382, t43385, t43398) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1365(t10250, t2970, t973, t10195, t10231, t1005, t10375, t10475, t42342, t42345, t2770, t283);
        let t43415 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1366(t43398, t61, t10309, t1041, t10457, t248, t10444, t354, t364, t372, t1021, t10364, t10408, t10413, t1046, t10482, t10962, t10965, t10972, t2771, t2960, t3041, t3057, t3064, t3117, t3123, t41667, t41715, t42348, t43374, t43377, t43382, t43385, t973, t977);
        let t43419 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1367(t42337, t42409, t42459, t42499, t42540, t42580, t42621, t42723, t43099, t43141, t43181, t43223, t43267, t43315, t43366, t43415);
        let t43447 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1368(t11018, t225, t3206, t11016, t10160, t10170, t10182, t10358, t1049, t1052, t1066, t11007, t11010, t11085, t3020, t3026, t3166, t3169, t3174, t3176, t3207, t349, t388, t43419, t990);
        let (t43470, t43473, t43480, t43483, t43489, t43503) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1369(t11064, t42332, t11058, t3185, t42741, t10481, t1049, t3040, t3166, t1014, t42340, t42341);
        let (t43504, t43512) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1370(t381, t42348, t23508, t360, t1003, t1022, t10359, t1058, t1060, t1063, t11007, t11027, t11031, t11043, t11065, t11066, t14590, t3180, t3186, t3188, t3189, t3196, t353, t383, t43419, t43480, t43483, t43489, t43503, t4673);
    (t43292, t43447, t43470, t43473, t43483, t43489, t43504, t43512)
}
