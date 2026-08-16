//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta736 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2596;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta736(t13969: f64, t15636: f64, t3515: f64, t1174: f64, t44571: f64, t4724: f64, t11778: f64, t43791: f64, t1227: f64, t49850: f64, t4988: f64, t15568: f64, t3604: f64, t11697: f64, t15473: f64, t3577: f64, t11698: f64, t15740: f64, t10401: f64, t15567: f64, t3610: f64, t11692: f64, t15563: f64, t15743: f64, t3490: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52586, t52599, t52601, t52609, t52615) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2596(t13969, t15636, t3515, t1174, t44571, t4724, t11778, t43791, t1227, t49850, t4988, t15568, t3604);
        let (t52619, t52621, t52627, t52628, t52649, t52653) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2597(t11697, t15473, t3577, t11698, t15740, t10401, t15567, t3610, t11692, t15563, t15743, t3490);
    (t52586, t52599, t52601, t52609, t52615, t52619, t52621, t52627, t52628, t52649, t52653)
}
