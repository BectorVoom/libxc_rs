//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta184 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1128;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1129;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1130;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1131;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1132;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta184(t4437: f64, t931: f64, t2766: f64, t2892: f64, t4335: f64, t4340: f64, t4345: f64, t4349: f64, t324: f64, t1573: f64, t942: f64, t1581: f64, t950: f64, t2824: f64, t2912: f64, t2919: f64, t4363: f64, t4371: f64, t4379: f64, t4381: f64, t4384: f64, t4387: f64, t4390: f64, t4393: f64, t951: f64, t1580: f64, t2932: f64, t1569: f64, t2856: f64, t2861: f64, t2886: f64, t2900: f64, t2905: f64, t2930: f64, t311: f64, t4353: f64, t4356: f64, t4358: f64, t4361: f64, t4398: f64, t4402: f64, t4408: f64, t4411: f64, t4416: f64, t4434: f64, t924: f64, t933: f64, t943: f64, t952: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4438, t4446) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1128(t4437, t931, t2766, t2892, t4335, t4340, t4345, t4349);
        let (t4447, t4449, t4454, t4471) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1129(t324, t4446, t1573, t942, t1581, t950, t2766, t2824, t2912, t2919, t4335, t4340, t4345, t4349, t4363, t4371, t4379, t4381, t4384, t4387, t4390, t4393);
        let t4472 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1130(t4471, t951);
        let t4475 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1131(t1580, t2932);
        let (t4476, t4479) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1132(t4475, t950, t1569, t1581, t2856, t2861, t2886, t2900, t2905, t2930, t311, t4353, t4356, t4358, t4361, t4398, t4402, t4408, t4411, t4416, t4434, t4438, t4447, t4449, t4454, t4472, t924, t933, t943, t952);
        let (t4480, t4482, t4483) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1133(t300, t4479, t4447, t1573);
    (t4438, t4446, t4449, t4454, t4471, t4472, t4475, t4476, t4480, t4482, t4483)
}
