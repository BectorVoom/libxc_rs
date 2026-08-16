//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2342;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2343;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2344;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta637(t1597: f64, t43052: f64, t2986: f64, t2990: f64, t10255: f64, t13847: f64, t10190: f64, t13861: f64, t13559: f64, t13779: f64, t10189: f64, t4540: f64, t42771: f64, t4514: f64, t43057: f64, t13913: f64, t2960: f64, t4542: f64, t698: f64, t973: f64, t10186: f64, t13788: f64, t13799: f64, t13858: f64, t13862: f64, t13865: f64, t13868: f64, t13877: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t48022, t48024, t48030, t48044, t48046) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2342(t1597, t43052, t2986, t2990, t10255, t13847, t10190, t13861, t13559, t13779, t10189, t4540);
        let (t48048, t48052, t48061, t48063, t48067, t48068) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2343(t2986, t2990, t48046, t42771, t4514, t43057, t13913, t2960, t4542, t698, t973, t10186, t13788);
        let t48076 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2344(t10186, t13799, t13858, t13862, t13865, t13868, t13877, t48052, t48061, t48063, t48067, t48068);
    (t48022, t48024, t48030, t48044, t48048, t48076)
}
