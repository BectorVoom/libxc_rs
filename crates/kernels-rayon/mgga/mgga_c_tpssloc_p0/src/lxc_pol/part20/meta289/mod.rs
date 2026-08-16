//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1494;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta289(t10868: f64, t248: f64, t884: f64, t1041: f64, t3048: f64, t3053: f64, t10478: f64, t3128: f64, t10472: f64, t10481: f64, t3131: f64, t1021: f64, t1015: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10870, t10871, t10873, t10875, t10876) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1494(t10868, t248, t884, t1041, t3048, t3053, t10478, t3128, t10472);
        let (t10877, t10879, t10882, t10883) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1495(t10481, t3131, t1021, t248, t1015, t10478, t10472);
    (t10870, t10871, t10873, t10875, t10876, t10877, t10879, t10882, t10883)
}
