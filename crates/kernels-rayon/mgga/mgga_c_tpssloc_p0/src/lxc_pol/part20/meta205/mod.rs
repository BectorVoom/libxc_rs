//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta205 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1217;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1218;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1219;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1220;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1221;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta205(t3961: f64, t4987: f64, t4582: f64, t1653: f64, t248: f64, t3521: f64, t1227: f64, t1735: f64, t3570: f64, t1213: f64, t1009: f64, t1720: f64, t1011: f64, t1212: f64, t1226: f64, t1730: f64, t1174: f64, t1218: f64, t1232: f64, t1737: f64, t3506: f64, t3515: f64, t3536: f64, t3577: f64, t488: f64, t4950: f64, t4954: f64, t4957: f64, t4959: f64, t4961: f64, t4966: f64, t4969: f64, t4974: f64, t4980: f64, t4984: f64, t4739: f64, t4742: f64, t4744: f64, t4747: f64, t4784: f64, t4788: f64, t4866: f64, t4868: f64, t4871: f64, t4873: f64, t4877: f64, t4881: f64, t4886: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4988, t4989, t4993, t4994, t4997, t4998, t5000) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1217(t3961, t4987, t4582, t1653, t248, t3521, t1227, t1735, t3570, t1213, t1009, t1720);
        let t5002 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1218(t1011, t5000, t1212);
        let t5005 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1219(t1226, t1730);
        let t5010 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1220(t1174, t1218, t1227, t1232, t1737, t3506, t3515, t3536, t3577, t488, t4950, t4954, t4957, t4959, t4961, t4966, t4969, t4974, t4980, t4984, t4989, t4994, t4998, t5002, t5005);
        let t5011 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1221(t4739, t4742, t4744, t4747, t4784, t4788, t4866, t4868, t4871, t4873, t4877, t4881, t4886);
        let t5012 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1222(t475, t5011);
    (t4988, t4989, t4993, t4997, t5000, t5002, t5005, t5010, t5011, t5012)
}
