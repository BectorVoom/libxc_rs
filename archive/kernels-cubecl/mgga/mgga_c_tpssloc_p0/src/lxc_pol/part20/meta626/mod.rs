//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta626 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2256;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2257;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2258;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2259;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2260;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2261;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2262;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2263;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2264;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta626<F: Float>(t41142: F, t41144: F, t41149: F, t41151: F, t41155: F, t41156: F, t41158: F, t41173: F, t41181: F, t41185: F, t41187: F, t12985: F, t9577: F, t212: F, t4119: F, t2586: F, t9523: F, t4138: F, t9541: F, t41189: F, t4134: F, t118: F, t12971: F, t2576: F, t794: F, t13025: F, t9546: F, t210: F, t214: F, t41190: F, t41192: F, t41194: F, t41197: F, t41200: F, t46426: F, t787: F, t13017: F, t2563: F, t1489: F, t41083: F, t2559: F, t4126: F, t4130: F, t12997: F, t13000: F, t2566: F, t67: F, t792: F, t9558: F, t12984: F, t2379: F, t686: F, t133: F, t1484: F, t41214: F, t6600: F, t12998: F, t776: F, t12988: F, t213: F, t221: F, t2553: F, t41203: F, t41205: F, t4127: F, t12990: F, t13012: F, t12994: F, t13196: F, t13004: F, t782: F, t13007: F, t131: F, t205: F, t41160: F, t116: F, t2570: F, t2585: F, t4255: F, t13005: F, t41209: F, t41212: F, t41217: F, t4128: F, t9458: F, t9516: F, t225: F, t13242: F, t13244: F, t13254: F, t13265: F, t13316: F, t16836: F, t237: F, t249: F, t2633: F, t2643: F, t2679: F, t2684: F, t41066: F, t4178: F, t4180: F, t4181: F, t46717: F, t46733: F, t46737: F, t46742: F, t46748: F, t9629: F, t9642: F, t9958: F, t13326: F, t9638: F, t2628: F, t2691: F, t4184: F, t812: F, t1512: F, t41362: F, t13176: F, t2629: F, t4166: F, t9666: F, t2635: F, t13337: F, t838: F, t2693: F, t4163: F, t13080: F, t13084: F, t13223: F, t13251: F, t13262: F, t13350: F, t1495: F, t2571: F, t2645: F, t4158: F, t4248: F, t9647: F, t9649: F, t9976: F) -> (F, F, F, F, F, F, F) {
        let (t46759, t46764) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2256::<F>(t41142, t41144, t41149, t41151, t41155, t41156, t41158, t41173, t41181, t41185, t41187, t12985, t9577);
        let (t46766, t46769, t46770, t46772, t46780) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2257::<F>(t212, t4119, t2586, t9523, t4138, t9541, t41189, t4134, t118, t12971, t2576, t794);
        let t46784 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2258::<F>(t13025, t9546, t210, t214, t41190, t41192, t41194, t41197, t41200, t46426, t46764, t46769, t46770, t46772, t46780, t787);
        let (t46788, t46790, t46794, t46796, t46799) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2259::<F>(t13017, t2563, t1489, t41083, t2559, t4126, t4130, t12997, t13000, t2566, t67, t792, t9558);
        let t46821 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2260::<F>(t12984, t2379, t46799, t686, t133, t1484, t41214, t6600, t12998, t46766, t776, t12971, t12988, t213, t221, t2553, t41203, t41205, t4127, t46788, t46790, t46794, t46796);
        let (t46828, t46830, t46836, t46838, t46839, t46843) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2261::<F>(t12984, t12998, t2553, t686, t12990, t13012, t12994, t213, t221, t13196, t776, t13004, t782);
        let (t46853, t46858) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2262::<F>(t13007, t46843, t131, t205, t41160, t116, t212, t2570, t2585, t4255, t12988, t13005, t221, t2379, t41209, t41212, t41217, t4127, t4128, t46828, t46830, t46836, t46838, t46839, t9458, t9516);
        let (t46860, t46861, t46868) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2263::<F>(t46759, t46784, t46821, t46858, t225, t13242, t13244, t13254, t13265, t13316, t16836, t237, t249, t2633, t2643, t2679, t2684, t41066, t4178, t4180, t4181, t46717, t46733, t46737, t46742, t46748, t9629, t9642, t9958);
        let (t46870, t46875, t46876, t46878, t46881) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2264::<F>(t13326, t9638, t2628, t2691, t4184, t812, t1512, t41362, t13176, t2629, t4166, t9666);
        let t46910 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2265::<F>(t2635, t46881, t13337, t838, t2693, t4163, t13080, t13084, t13223, t13251, t13254, t13262, t13350, t1495, t210, t2553, t2571, t2643, t2645, t4158, t4248, t46870, t46875, t46876, t46878, t9516, t9642, t9647, t9649, t9976);
    (t46838, t46839, t46853, t46860, t46861, t46868, t46910)
}
