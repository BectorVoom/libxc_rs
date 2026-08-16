//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta861 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3122;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3123;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3124;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3125;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3126;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3127;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta861(t19262: f64, t3640: f64, t1164: f64, t3400: f64, t3403: f64, t63283: f64, t1156: f64, t3375: f64, t18276: f64, t3411: f64, t11126: f64, t6106: f64, t18287: f64, t225: f64, t11925: f64, t11928: f64, t1235: f64, t1252: f64, t14980: f64, t15771: f64, t15789: f64, t15790: f64, t15797: f64, t15803: f64, t1720: f64, t1761: f64, t18571: f64, t19209: f64, t19249: f64, t27784: f64, t3590: f64, t3593: f64, t3600: f64, t4945: f64, t498: f64, t5055: f64, t5089: f64, t53677: f64, t53703: f64, t6150: f64, t6244: f64, t6268: f64, t15419: f64, t18215: f64, t3447: f64, t18469: f64, t44525: f64, t18206: f64, t52133: f64, t15324: f64, t15327: f64, t15376: f64, t15379: f64, t15391: f64, t44529: f64, t44558: f64, t4900: f64, t63386: f64, t63394: f64, t4899: f64, t6138: f64, t6144: f64, t11571: f64, t15313: f64, t15320: f64, t15396: f64, t4904: f64, t4919: f64, t51948: f64, t51961: f64, t51970: f64, t51980: f64, t51988: f64, t51991: f64, t51995: f64, t52040: f64, t15420: f64, t18211: f64, t11575: f64, t11579: f64, t11584: f64, t15268: f64, t15321: f64, t18409: f64, t18416: f64, t18420: f64, t4908: f64, t51975: f64, t52013: f64, t63298: f64, t63302: f64, t15402: f64, t18225: f64, t11589: f64, t18427: f64, t18221: f64, t15399: f64, t15403: f64, t11593: f64, t15314: f64, t15332: f64, t15335: f64, t15395: f64, t15415: f64, t63415: f64, t15339: f64, t18232: f64, t15317: f64, t52019: f64, t52022: f64, t52038: f64, t52050: f64, t52053: f64, t52057: f64, t52061: f64, t52064: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64548, t64558, t64562, t64564, t64566) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3122(t19262, t3640, t1164, t3400, t3403, t63283, t1156, t3375, t18276, t3411, t11126, t6106);
        let t64602 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3123(t18287, t225, t11925, t11928, t1235, t1252, t14980, t15771, t15789, t15790, t15797, t15803, t1720, t1761, t18571, t19209, t19249, t27784, t3590, t3593, t3600, t4945, t498, t5055, t5089, t53677, t53703, t6150, t6244, t6268);
        let t64634 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3124(t15419, t18215, t3447, t18469, t44525, t18206, t52133, t15324, t15327, t15376, t15379, t15391, t44529, t44558, t4900, t63386, t63394);
        let t64660 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3125(t4899, t6138, t6144, t11571, t15313, t15320, t15376, t15396, t3447, t4904, t4919, t51948, t51961, t51970, t51980, t51988, t51991, t51995, t52040);
        let t64694 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3126(t15376, t15420, t15419, t18211, t3447, t11575, t11579, t11584, t15268, t15321, t18409, t18416, t18420, t4908, t51975, t52013, t63298, t63302);
        let t64725 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3127(t15402, t18225, t3447, t11589, t18427, t18221, t15376, t15399, t15403, t18409, t11593, t15314, t15332, t15335, t15395, t15415, t63415);
        let t64746 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3128(t15339, t15376, t15419, t18232, t3447, t11593, t15317, t18427, t52019, t52022, t52038, t52050, t52053, t52057, t52061, t52064);
    (t64548, t64558, t64562, t64564, t64566, t64602, t64634, t64660, t64694, t64725, t64746)
}
