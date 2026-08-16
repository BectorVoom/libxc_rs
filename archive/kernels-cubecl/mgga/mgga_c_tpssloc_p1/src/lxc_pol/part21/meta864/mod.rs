//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta864 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3151;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3152;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3153;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3154;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3155;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta864<F: Float>(t63290: F, t64477: F, t64479: F, t64481: F, t64485: F, t64489: F, t64492: F, t64496: F, t64499: F, t64501: F, t64504: F, t64507: F, t64509: F, t63446: F, t63449: F, t63451: F, t63557: F, t64514: F, t64517: F, t64520: F, t64522: F, t64524: F, t64528: F, t64530: F, t64533: F, t63560: F, t63563: F, t63566: F, t63568: F, t63571: F, t63574: F, t63576: F, t63579: F, t63582: F, t63585: F, t63587: F, t63591: F, t63594: F, t63714: F, t63717: F, t63720: F, t63722: F, t63725: F, t63729: F, t64536: F, t64540: F, t64558: F, t64562: F, t64564: F, t64566: F, t63731: F, t63733: F, t63735: F, t63737: F, t63739: F, t63741: F, t63743: F, t63745: F, t63747: F, t63752: F, t63754: F, t63757: F, t63759: F, t18710: F, t300: F, t1166: F, t1164: F, t3396: F, t6105: F, t18933: F, t63763: F, t63765: F, t63767: F, t63769: F, t63771: F, t63829: F, t64100: F, t64253: F, t64259: F, t64433: F) -> (F, F, F, F, F, F, F, F, F) {
        let t65279 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3151::<F>(t63290, t64477, t64479, t64481, t64485, t64489, t64492, t64496, t64499, t64501, t64504, t64507, t64509);
        let t65281 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3152::<F>(t63446, t63449, t63451, t63557, t64514, t64517, t64520, t64522, t64524, t64528, t64530, t64533);
        let t65282 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3153::<F>(t63560, t63563, t63566, t63568, t63571, t63574, t63576, t63579, t63582, t63585, t63587, t63591, t63594);
        let t65285 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3154::<F>(t63714, t63717, t63720, t63722, t63725, t63729, t64536, t64540, t64558, t64562, t64564, t64566);
        let t65286 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3155::<F>(t63731, t63733, t63735, t63737, t63739, t63741, t63743, t63745, t63747, t63752, t63754, t63757, t63759);
        let (t65290, t65293, t65296, t65297) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3156::<F>(t18710, t300, t1166, t1164, t3396, t6105, t18933, t63763, t63765, t63767, t63769, t63771, t63829, t64100, t64253, t64259, t64433);
    (t65279, t65281, t65282, t65285, t65286, t65290, t65293, t65296, t65297)
}
