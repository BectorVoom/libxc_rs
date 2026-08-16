//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk672;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk673;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk674;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta140(t324: f64, t5769: f64, t1580: f64, t951: f64, t2912: f64, t2919: f64, t4335: f64, t4384: f64, t5679: f64, t5683: f64, t5687: f64, t5699: f64, t5706: f64, t5712: f64, t5714: f64, t5718: f64, t5721: f64, t5724: f64, t2932: f64, t1569: f64, t1581: f64, t2861: f64, t2886: f64, t2905: f64, t2930: f64, t311: f64, t4411: f64, t4449: f64, t5691: f64, t5693: f64, t5697: f64, t5729: f64, t5732: f64, t5737: f64, t5743: f64, t5759: f64, t5762: f64, t924: f64, t943: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5770, t5774) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk672(t324, t5769, t1580);
        let (t5775, t5790) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk673(t5774, t951, t2912, t2919, t4335, t4384, t5679, t5683, t5687, t5699, t5706, t5712, t5714, t5718, t5721, t5724);
        let t5791 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk674(t5790, t951);
        let (t5794, t5797) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk675(t2932, t5774, t1569, t1581, t2861, t2886, t2905, t2930, t311, t4411, t4449, t5691, t5693, t5697, t5729, t5732, t5737, t5743, t5759, t5762, t5770, t5775, t5791, t924, t943);
    (t5770, t5774, t5775, t5790, t5791, t5794, t5797)
}
