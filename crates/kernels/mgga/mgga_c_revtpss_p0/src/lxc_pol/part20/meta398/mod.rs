//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1473;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1474;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1475;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1476;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1477;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta398<F: Float>(t11299: F, t2918: F, t2927: F, t11380: F, t2874: F, t934: F, t11379: F, t2924: F, t2926: F, t11294: F, t11531: F, t41500: F, t935: F, t41510: F, t2866: F, t2873: F, t2876: F, t11298: F, t910: F, t11301: F, t11385: F, t11452: F, t2962: F, t41306: F, t41308: F, t41312: F, t41316: F, t41320: F, t41323: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41341: F, t41344: F, t41347: F, t41350: F, t41353: F, t41356: F, t41359: F, t41361: F, t41363: F, t41365: F, t41367: F, t41369: F, t11404: F, t11409: F, t11444: F, t11450: F, t11517: F, t11521: F, t11551: F, t11554: F, t2943: F, t2944: F, t2968: F, t2970: F, t311: F, t41540: F, t41668: F, t41763: F, t953: F, t954: F, t300: F, t41778: F, t41825: F, t41853: F, t3333: F, t3335: F, t11598: F, t3022: F, t198: F, t336: F, t41577: F, t41580: F, t41582: F, t41585: F, t41591: F, t41657: F, t41841: F, t41845: F, t41847: F, t41849: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41864, t41867, t41871, t41873, t41876) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1473::<F>(t11299, t2918, t2927, t11380, t2874, t934, t11379, t2924, t2926, t11294, t11531, t41500, t935);
        let (t41879, t41882, t41885, t41888) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1474::<F>(t2874, t41510, t935, t2866, t2873, t2876, t11298, t910, t11301, t11385, t2926, t41500);
        let (t41895, t41913) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1475::<F>(t11452, t2962, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t41926 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1476::<F>(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let t41930 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1477::<F>(t11404, t11409, t11444, t11450, t11517, t11521, t11551, t11554, t2943, t2944, t2968, t2970, t311, t41540, t41668, t41763, t41864, t41867, t41871, t41873, t41876, t41879, t41882, t41885, t41888, t41895, t41913, t41926, t953, t954);
        let (t41933, t41942, t41943) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1478::<F>(t300, t41778, t41825, t41853, t41930, t3333, t3335, t11598, t3022, t198, t336, t41577, t41580, t41582, t41585, t41591, t41657, t41841, t41845, t41847, t41849);
    (t41864, t41867, t41871, t41873, t41876, t41879, t41882, t41885, t41888, t41933, t41942, t41943)
}
