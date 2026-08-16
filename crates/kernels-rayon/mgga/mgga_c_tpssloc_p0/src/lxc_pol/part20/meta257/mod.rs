//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1389;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta257(t252: f64, t2678: f64, t829: f64, t860: f64, t9661: f64, t10016: f64, t10055: f64, t10058: f64, t10069: f64, t10073: f64, t10077: f64, t10081: f64, t10084: f64, t10091: f64, t10094: f64, t226: f64, t255: f64, t2613: f64, t2617: f64, t2729: f64, t2733: f64, t2736: f64, t2738: f64, t2740: f64, t4281: f64, t4291: f64, t808: f64, t812: f64, t861: f64, t863: f64, t9612: f64, t858: f64, t856: f64, t68: f64, t2719: f64, t865: f64, t2742: f64, t2718: f64, t10047: f64, t10049: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t2743: f64, t855: f64, t866: f64, t9520: f64, t9585: f64, t9587: f64, t9590: f64, t9593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10098, t10101, t10103) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1389(t252, t2678, t829, t860, t9661, t10016, t10055, t10058, t10069, t10073, t10077, t10081, t10084, t10091, t10094, t226, t255, t2613, t2617, t2729, t2733, t2736, t2738, t2740, t4281, t4291, t808, t812, t861, t863, t9612);
        let (t10104, t10108, t10109, t10110, t10111, t10112, t10115, t10116, t10121) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1390(t10103, t858, t856, t68, t2719, t865, t2742, t2718, t10047, t10049, t259, t2597, t2713, t2720, t2743, t855, t866, t9520, t9585, t9587, t9590, t9593);
    (t10098, t10101, t10103, t10104, t10108, t10109, t10110, t10111, t10112, t10115, t10116, t10121)
}
