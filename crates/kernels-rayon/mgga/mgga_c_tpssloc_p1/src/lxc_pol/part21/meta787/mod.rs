//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta787 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2738;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2739;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2740;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2741;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2742;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2743;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta787(t1799: f64, t3698: f64, t20063: f64, t3701: f64, t1388: f64, t15899: f64, t3918: f64, t39642: f64, t39655: f64, t39658: f64, t5160: f64, t57206: f64, t57207: f64, t57209: f64, t57210: f64, t57212: f64, t57213: f64, t57214: f64, t20085: f64, t3914: f64, t39844: f64, t57215: f64, t57216: f64, t57218: f64, t57219: f64, t57220: f64, t57221: f64, t57222: f64, t57223: f64, t57224: f64, t57225: f64, t12470: f64, t193: f64, t3924: f64, t40224: f64, t40230: f64, t56486: f64, t57226: f64, t57228: f64, t57230: f64, t57231: f64, t57232: f64, t57233: f64, t57236: f64, t57237: f64, t6330: f64, t40: f64, t12606: f64, t12652: f64, t12862: f64, t16549: f64, t16554: f64, t16558: f64, t2244: f64, t2250: f64, t2433: f64, t40632: f64, t4080: f64, t5392: f64, t5398: f64, t55677: f64, t55723: f64, t607: f64, t73: f64, t9427: f64, zeta_threshold: f64, t52: f64, t12874: f64, t16563: f64, t16568: f64, t2440: f64, t40647: f64, t4087: f64, t76: f64, t9438: f64, t157: f64, t182: f64, t145: f64, t185: f64, t46125: f64, t46128: f64, t46130: f64, t16576: f64, t751: f64, t46132: f64, t16583: f64, t16589: f64, t39249: f64, t39256: f64, t40626: f64, t46341: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t57810 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2738(t1799, t3698, t20063, t3701, t1388, t15899, t3918, t39642, t39655, t39658, t5160, t57206, t57207, t57209, t57210, t57212, t57213, t57214);
        let t57815 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2739(t20085, t3914, t39844, t5160, t57215, t57216, t57218, t57219, t57220, t57221, t57222, t57223, t57224, t57225);
        let t57822 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2740(t12470, t193, t3924, t40224, t40230, t56486, t57226, t57228, t57230, t57231, t57232, t57233, t57236, t57237, t6330);
        let t57850 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2741(t40, t12606, t12652, t12862, t16549, t16554, t16558, t2244, t2250, t2433, t40632, t4080, t5392, t5398, t55677, t55723, t607, t73, t9427, zeta_threshold);
        let t57873 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2742(t52, t12606, t12652, t12874, t16558, t16563, t16568, t2244, t2250, t2440, t40647, t4087, t5392, t5398, t55677, t55723, t607, t76, t9438, zeta_threshold);
        let (t57877, t57879, t57880, t57885, t57886, t57888, t57889) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2743(t57850, t57873, t157, t182, t145, t185, t46125, t46128, t46130, t16576, t751, t46132);
        let t57890 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2744(t16583, t16589, t39249, t39256, t40626, t46341, t57877, t57879, t57880, t57885, t57886, t57888, t57889);
    (t57810, t57815, t57822, t57877, t57879, t57880, t57885, t57886, t57888, t57889, t57890)
}
