//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1463;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1464;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta279(t10588: f64, t901: f64, t276: f64, t285: f64, t2799: f64, t896: f64, t273: f64, t10311: f64, t10318: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64, t10589: f64, t10553: f64, t942: f64, t951: f64, t959: f64, t10544: f64, t10530: f64, t10538: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10591, t10595, t10596, t10597, t10599, t10600, t10602) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1463(t10588, t901, t276, t285, t2799, t896, t273, t10311, t10318, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575, t10589);
        let t10603 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1464(t10553, t10602);
        let (t10605, t10607, t10608, t10619) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1465(t10603, t942, t951, t959, t10544, t10530, t10538, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575);
    (t10591, t10595, t10596, t10597, t10599, t10600, t10603, t10605, t10607, t10608, t10619)
}
