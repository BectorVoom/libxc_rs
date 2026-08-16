//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1759;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1760;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1761;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta388(t2684: f64, t4295: f64, t13171: f64, t860: f64, t4265: f64, t814: f64, t829: f64, t13377: f64, t235: f64, t2679: f64, t4282: f64, t4280: f64, t808: f64, t13384: f64, t13176: f64, t13336: f64, t1499: f64, t1523: f64, t1525: f64, t226: f64, t255: f64, t2613: f64, t2617: f64, t2738: f64, t2740: f64, t4162: f64, t4166: f64, t4283: f64, t4286: f64, t4288: f64, t4291: f64, t4298: f64, t812: f64, t861: f64, t863: f64, t9612: f64, t13425: f64, t858: f64, t225: f64, t4149: f64, t13050: f64, t13053: f64, t13059: f64, t13062: f64, t13065: f64, t13068: f64, t13072: f64, t13378: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t4268: f64, t4273: f64, t4301: f64, t855: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13429, t13431, t13433, t13434, t13448, t13450, t13453) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1759(t2684, t4295, t13171, t860, t4265, t814, t829, t13377, t235, t2679, t4282, t4280, t808);
        let (t13456, t13459) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1760(t13384, t829, t13176, t13336, t13429, t13431, t13434, t13448, t13450, t13453, t1499, t1523, t1525, t226, t255, t2613, t2617, t2738, t2740, t4162, t4166, t4283, t4286, t4288, t4291, t4298, t808, t812, t861, t863, t9612);
        let (t13460, t13461, t13463, t13470) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1761(t13425, t13459, t858, t225, t4149, t13050, t13053, t13059, t13062, t13065, t13068, t13072, t13378, t259, t2597, t2713, t2720, t4268, t4273, t4301, t855, t866);
    (t13429, t13431, t13433, t13434, t13448, t13450, t13453, t13456, t13460, t13461, t13463, t13470)
}
