//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1389;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta257<F: Float>(t252: F, t2678: F, t829: F, t860: F, t9661: F, t10016: F, t10055: F, t10058: F, t10069: F, t10073: F, t10077: F, t10081: F, t10084: F, t10091: F, t10094: F, t226: F, t255: F, t2613: F, t2617: F, t2729: F, t2733: F, t2736: F, t2738: F, t2740: F, t4281: F, t4291: F, t808: F, t812: F, t861: F, t863: F, t9612: F, t858: F, t856: F, t68: F, t2719: F, t865: F, t2742: F, t2718: F, t10047: F, t10049: F, t259: F, t2597: F, t2713: F, t2720: F, t2743: F, t855: F, t866: F, t9520: F, t9585: F, t9587: F, t9590: F, t9593: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10098, t10101, t10103) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1389::<F>(t252, t2678, t829, t860, t9661, t10016, t10055, t10058, t10069, t10073, t10077, t10081, t10084, t10091, t10094, t226, t255, t2613, t2617, t2729, t2733, t2736, t2738, t2740, t4281, t4291, t808, t812, t861, t863, t9612);
        let (t10104, t10108, t10109, t10110, t10111, t10112, t10115, t10116, t10121) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1390::<F>(t10103, t858, t856, t68, t2719, t865, t2742, t2718, t10047, t10049, t259, t2597, t2713, t2720, t2743, t855, t866, t9520, t9585, t9587, t9590, t9593);
    (t10098, t10101, t10103, t10104, t10108, t10109, t10110, t10111, t10112, t10115, t10116, t10121)
}
