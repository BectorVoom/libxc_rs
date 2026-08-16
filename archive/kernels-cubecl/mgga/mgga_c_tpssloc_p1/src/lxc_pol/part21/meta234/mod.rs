//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta234 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1393;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1394;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1395;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1396;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1397;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta234<F: Float>(t5774: F, t951: F, t2912: F, t2919: F, t4335: F, t4384: F, t5679: F, t5683: F, t5687: F, t5699: F, t5706: F, t5712: F, t5714: F, t5718: F, t5721: F, t5724: F, t2932: F, t1569: F, t1581: F, t2861: F, t2886: F, t2905: F, t2930: F, t311: F, t4411: F, t4449: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5737: F, t5743: F, t5759: F, t5762: F, t5770: F, t924: F, t943: F, t300: F, t1589: F, t4483: F, t2904: F, t959: F, t942: F, t2929: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5775 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1393::<F>(t5774, t951);
        let t5790 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1394::<F>(t2912, t2919, t4335, t4384, t5679, t5683, t5687, t5699, t5706, t5712, t5714, t5718, t5721, t5724);
        let t5791 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1395::<F>(t5790, t951);
        let t5794 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1396::<F>(t2932, t5774);
        let t5797 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1397::<F>(t1569, t1581, t2861, t2886, t2905, t2930, t311, t4411, t4449, t5691, t5693, t5697, t5729, t5732, t5737, t5743, t5759, t5762, t5770, t5775, t5791, t5794, t924, t943);
        let (t5798, t5800, t5802, t5804, t5806, t5808, t5810, t5811) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1398::<F>(t300, t5797, t5770, t1589, t4483, t2904, t5774, t951, t959, t5790, t942, t2929);
    (t5775, t5790, t5791, t5794, t5798, t5800, t5802, t5804, t5806, t5808, t5810, t5811)
}
