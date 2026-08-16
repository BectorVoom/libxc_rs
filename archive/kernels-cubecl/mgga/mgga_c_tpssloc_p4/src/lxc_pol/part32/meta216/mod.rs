//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1024;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1025;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta216<F: Float>(t5758: F, t932: F, t2888: F, t5742: F, t2892: F, t4335: F, t5679: F, t5683: F, t5687: F, t324: F, t1580: F, t951: F, t2912: F, t2919: F, t4384: F, t5699: F, t5706: F, t5712: F, t5714: F, t5718: F, t5721: F, t5724: F, t2932: F, t1569: F, t1581: F, t2861: F, t2886: F, t2905: F, t2930: F, t311: F, t4411: F, t4449: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5737: F, t5743: F, t924: F, t943: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5759, t5762, t5769, t5770, t5774) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1024::<F>(t5758, t932, t2888, t5742, t2892, t4335, t5679, t5683, t5687, t324, t1580);
        let (t5775, t5790) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1025::<F>(t5774, t951, t2912, t2919, t4335, t4384, t5679, t5683, t5687, t5699, t5706, t5712, t5714, t5718, t5721, t5724);
        let (t5791, t5794, t5797) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1026::<F>(t5790, t951, t2932, t5774, t1569, t1581, t2861, t2886, t2905, t2930, t311, t4411, t4449, t5691, t5693, t5697, t5729, t5732, t5737, t5743, t5759, t5762, t5770, t5775, t924, t943);
    (t5759, t5762, t5769, t5770, t5774, t5775, t5790, t5791, t5794, t5797)
}
