//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1309;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1310;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1311;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1312;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta361(t1041: f64, t10918: f64, t13969: f64, t607: f64, t9258: f64, t10403: f64, t10404: f64, t10422: f64, t10477: f64, t67: f64, t3067: f64, t11059: f64, t10970: f64, t820: f64, t10418: f64, t3070: f64, t1021: f64, t1023: f64, t10305: f64, t10316: f64, t10321: f64, t10408: f64, t10426: f64, t10483: f64, t10883: f64, t10886: f64, t248: f64, t2771: f64, t3041: f64, t3071: f64, t3131: f64, t3132: f64, t360: f64, t42347: f64, t42348: f64, t42354: f64, t42358: f64, t42369: f64, t4582: f64, t4583: f64, t884: f64, t10397: f64, t3120: f64, t10517: f64, t3103: f64, t10868: f64, t2780: f64, t3051: f64, t10277: f64, t976: f64, t10263: f64, t10493: f64, t2776: f64, t3039: f64, t3048: f64, t3121: f64, t3146: f64, t3151: f64, t3153: f64, t39097: f64, t39103: f64, t4588: f64, t973: f64, t974: f64, t10993: f64, t2960: f64, t2244: f64, t2250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42372, t42374, t42380, t42386, t42387, t42388) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1309(t1041, t10918, t13969, t607, t9258, t10403, t10404, t10422, t10477, t67, t3067, t11059);
        let t42409 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1310(t10970, t820, t10418, t10422, t3070, t1021, t1023, t10305, t10316, t10321, t10403, t10408, t1041, t10426, t10483, t10883, t10886, t248, t2771, t3041, t3071, t3131, t3132, t360, t42347, t42348, t42354, t42358, t42369, t42372, t42374, t42380, t42388, t4582, t4583, t884);
        let (t42412, t42422, t42428, t42432, t42436) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1311(t10397, t10422, t3070, t3120, t10517, t3103, t1041, t10868, t248, t2780, t10316, t3051);
        let t42459 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1312(t10277, t976, t1021, t10263, t10403, t1041, t10493, t248, t2776, t3039, t3048, t3070, t3071, t3121, t3132, t3146, t3151, t3153, t360, t39097, t39103, t42374, t42412, t42422, t42428, t42432, t42436, t4582, t4588, t973, t974);
        let (t42460, t42468) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1313(t10993, t2960, t2244, t2250);
    (t42374, t42386, t42387, t42409, t42422, t42459, t42460, t42468)
}
