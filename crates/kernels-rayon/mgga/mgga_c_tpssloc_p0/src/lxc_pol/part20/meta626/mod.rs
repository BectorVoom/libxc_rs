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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta626(t41142: f64, t41144: f64, t41149: f64, t41151: f64, t41155: f64, t41156: f64, t41158: f64, t41173: f64, t41181: f64, t41185: f64, t41187: f64, t12985: f64, t9577: f64, t212: f64, t4119: f64, t2586: f64, t9523: f64, t4138: f64, t9541: f64, t41189: f64, t4134: f64, t118: f64, t12971: f64, t2576: f64, t794: f64, t13025: f64, t9546: f64, t210: f64, t214: f64, t41190: f64, t41192: f64, t41194: f64, t41197: f64, t41200: f64, t46426: f64, t787: f64, t13017: f64, t2563: f64, t1489: f64, t41083: f64, t2559: f64, t4126: f64, t4130: f64, t12997: f64, t13000: f64, t2566: f64, t67: f64, t792: f64, t9558: f64, t12984: f64, t2379: f64, t686: f64, t133: f64, t1484: f64, t41214: f64, t6600: f64, t12998: f64, t776: f64, t12988: f64, t213: f64, t221: f64, t2553: f64, t41203: f64, t41205: f64, t4127: f64, t12990: f64, t13012: f64, t12994: f64, t13196: f64, t13004: f64, t782: f64, t13007: f64, t131: f64, t205: f64, t41160: f64, t116: f64, t2570: f64, t2585: f64, t4255: f64, t13005: f64, t41209: f64, t41212: f64, t41217: f64, t4128: f64, t9458: f64, t9516: f64, t225: f64, t13242: f64, t13244: f64, t13254: f64, t13265: f64, t13316: f64, t16836: f64, t237: f64, t249: f64, t2633: f64, t2643: f64, t2679: f64, t2684: f64, t41066: f64, t4178: f64, t4180: f64, t4181: f64, t46717: f64, t46733: f64, t46737: f64, t46742: f64, t46748: f64, t9629: f64, t9642: f64, t9958: f64, t13326: f64, t9638: f64, t2628: f64, t2691: f64, t4184: f64, t812: f64, t1512: f64, t41362: f64, t13176: f64, t2629: f64, t4166: f64, t9666: f64, t2635: f64, t13337: f64, t838: f64, t2693: f64, t4163: f64, t13080: f64, t13084: f64, t13223: f64, t13251: f64, t13262: f64, t13350: f64, t1495: f64, t2571: f64, t2645: f64, t4158: f64, t4248: f64, t9647: f64, t9649: f64, t9976: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t46759, t46764) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2256(t41142, t41144, t41149, t41151, t41155, t41156, t41158, t41173, t41181, t41185, t41187, t12985, t9577);
        let (t46766, t46769, t46770, t46772, t46780) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2257(t212, t4119, t2586, t9523, t4138, t9541, t41189, t4134, t118, t12971, t2576, t794);
        let t46784 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2258(t13025, t9546, t210, t214, t41190, t41192, t41194, t41197, t41200, t46426, t46764, t46769, t46770, t46772, t46780, t787);
        let (t46788, t46790, t46794, t46796, t46799) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2259(t13017, t2563, t1489, t41083, t2559, t4126, t4130, t12997, t13000, t2566, t67, t792, t9558);
        let t46821 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2260(t12984, t2379, t46799, t686, t133, t1484, t41214, t6600, t12998, t46766, t776, t12971, t12988, t213, t221, t2553, t41203, t41205, t4127, t46788, t46790, t46794, t46796);
        let (t46828, t46830, t46836, t46838, t46839, t46843) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2261(t12984, t12998, t2553, t686, t12990, t13012, t12994, t213, t221, t13196, t776, t13004, t782);
        let (t46853, t46858) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2262(t13007, t46843, t131, t205, t41160, t116, t212, t2570, t2585, t4255, t12988, t13005, t221, t2379, t41209, t41212, t41217, t4127, t4128, t46828, t46830, t46836, t46838, t46839, t9458, t9516);
        let (t46860, t46861, t46868) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2263(t46759, t46784, t46821, t46858, t225, t13242, t13244, t13254, t13265, t13316, t16836, t237, t249, t2633, t2643, t2679, t2684, t41066, t4178, t4180, t4181, t46717, t46733, t46737, t46742, t46748, t9629, t9642, t9958);
        let (t46870, t46875, t46876, t46878, t46881) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2264(t13326, t9638, t2628, t2691, t4184, t812, t1512, t41362, t13176, t2629, t4166, t9666);
        let t46910 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2265(t2635, t46881, t13337, t838, t2693, t4163, t13080, t13084, t13223, t13251, t13254, t13262, t13350, t1495, t210, t2553, t2571, t2643, t2645, t4158, t4248, t46870, t46875, t46876, t46878, t9516, t9642, t9647, t9649, t9976);
    (t46838, t46839, t46853, t46860, t46861, t46868, t46910)
}
