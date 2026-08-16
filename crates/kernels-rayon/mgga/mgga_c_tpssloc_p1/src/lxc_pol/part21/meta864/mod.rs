//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta864 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3151;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3152;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3153;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3154;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3155;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta864(t63290: f64, t64477: f64, t64479: f64, t64481: f64, t64485: f64, t64489: f64, t64492: f64, t64496: f64, t64499: f64, t64501: f64, t64504: f64, t64507: f64, t64509: f64, t63446: f64, t63449: f64, t63451: f64, t63557: f64, t64514: f64, t64517: f64, t64520: f64, t64522: f64, t64524: f64, t64528: f64, t64530: f64, t64533: f64, t63560: f64, t63563: f64, t63566: f64, t63568: f64, t63571: f64, t63574: f64, t63576: f64, t63579: f64, t63582: f64, t63585: f64, t63587: f64, t63591: f64, t63594: f64, t63714: f64, t63717: f64, t63720: f64, t63722: f64, t63725: f64, t63729: f64, t64536: f64, t64540: f64, t64558: f64, t64562: f64, t64564: f64, t64566: f64, t63731: f64, t63733: f64, t63735: f64, t63737: f64, t63739: f64, t63741: f64, t63743: f64, t63745: f64, t63747: f64, t63752: f64, t63754: f64, t63757: f64, t63759: f64, t18710: f64, t300: f64, t1166: f64, t1164: f64, t3396: f64, t6105: f64, t18933: f64, t63763: f64, t63765: f64, t63767: f64, t63769: f64, t63771: f64, t63829: f64, t64100: f64, t64253: f64, t64259: f64, t64433: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t65279 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3151(t63290, t64477, t64479, t64481, t64485, t64489, t64492, t64496, t64499, t64501, t64504, t64507, t64509);
        let t65281 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3152(t63446, t63449, t63451, t63557, t64514, t64517, t64520, t64522, t64524, t64528, t64530, t64533);
        let t65282 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3153(t63560, t63563, t63566, t63568, t63571, t63574, t63576, t63579, t63582, t63585, t63587, t63591, t63594);
        let t65285 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3154(t63714, t63717, t63720, t63722, t63725, t63729, t64536, t64540, t64558, t64562, t64564, t64566);
        let t65286 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3155(t63731, t63733, t63735, t63737, t63739, t63741, t63743, t63745, t63747, t63752, t63754, t63757, t63759);
        let (t65290, t65293, t65296, t65297) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3156(t18710, t300, t1166, t1164, t3396, t6105, t18933, t63763, t63765, t63767, t63769, t63771, t63829, t64100, t64253, t64259, t64433);
    (t65279, t65281, t65282, t65285, t65286, t65290, t65293, t65296, t65297)
}
