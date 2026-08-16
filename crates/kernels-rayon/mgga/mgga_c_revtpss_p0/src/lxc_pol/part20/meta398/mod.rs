//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1473;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1474;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1475;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1476;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1477;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta398(t11299: f64, t2918: f64, t2927: f64, t11380: f64, t2874: f64, t934: f64, t11379: f64, t2924: f64, t2926: f64, t11294: f64, t11531: f64, t41500: f64, t935: f64, t41510: f64, t2866: f64, t2873: f64, t2876: f64, t11298: f64, t910: f64, t11301: f64, t11385: f64, t11452: f64, t2962: f64, t41306: f64, t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41353: f64, t41356: f64, t41359: f64, t41361: f64, t41363: f64, t41365: f64, t41367: f64, t41369: f64, t11404: f64, t11409: f64, t11444: f64, t11450: f64, t11517: f64, t11521: f64, t11551: f64, t11554: f64, t2943: f64, t2944: f64, t2968: f64, t2970: f64, t311: f64, t41540: f64, t41668: f64, t41763: f64, t953: f64, t954: f64, t300: f64, t41778: f64, t41825: f64, t41853: f64, t3333: f64, t3335: f64, t11598: f64, t3022: f64, t198: f64, t336: f64, t41577: f64, t41580: f64, t41582: f64, t41585: f64, t41591: f64, t41657: f64, t41841: f64, t41845: f64, t41847: f64, t41849: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41864, t41867, t41871, t41873, t41876) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1473(t11299, t2918, t2927, t11380, t2874, t934, t11379, t2924, t2926, t11294, t11531, t41500, t935);
        let (t41879, t41882, t41885, t41888) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1474(t2874, t41510, t935, t2866, t2873, t2876, t11298, t910, t11301, t11385, t2926, t41500);
        let (t41895, t41913) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1475(t11452, t2962, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t41926 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1476(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let t41930 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1477(t11404, t11409, t11444, t11450, t11517, t11521, t11551, t11554, t2943, t2944, t2968, t2970, t311, t41540, t41668, t41763, t41864, t41867, t41871, t41873, t41876, t41879, t41882, t41885, t41888, t41895, t41913, t41926, t953, t954);
        let (t41933, t41942, t41943) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1478(t300, t41778, t41825, t41853, t41930, t3333, t3335, t11598, t3022, t198, t336, t41577, t41580, t41582, t41585, t41591, t41657, t41841, t41845, t41847, t41849);
    (t41864, t41867, t41871, t41873, t41876, t41879, t41882, t41885, t41888, t41933, t41942, t41943)
}
