//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta788 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2745;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta788(t46134: f64, t46137: f64, t4303: f64, t776: f64, t2517: f64, t5520: f64, t40667: f64, t40673: f64, t40680: f64, t2522: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40679: f64, t4307: f64, t40682: f64, t40687: f64, t46196: f64, t1484: f64, t2752: f64, t13487: f64, t2749: f64, t12854: f64, t12915: f64, t13196: f64, t1530: f64, t16596: f64, t16944: f64, t17116: f64, t17120: f64, t1877: f64, t193: f64, t200: f64, t2523: f64, t2745: f64, t39373: f64, t40685: f64, t4310: f64, t4314: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57891, t57892, t57897, t57898, t57899, t57900, t57901) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2745(t46134, t46137, t4303, t776, t2517, t5520, t40667, t40673, t40680, t2522, t39309, t39312, t39316, t39320, t40679, t4307);
        let (t57903, t57907, t57908, t57931) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2746(t40682, t40687, t46196, t1484, t2752, t13487, t2749, t12854, t12915, t13196, t1530, t16596, t16944, t17116, t17120, t1877, t193, t200, t2522, t2523, t2745, t39373, t40685, t4310, t4314);
    (t57891, t57892, t57897, t57898, t57899, t57900, t57901, t57903, t57907, t57908, t57931)
}
