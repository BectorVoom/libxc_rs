//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2235;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta676(t17884: f64, t3048: f64, t1009: f64, t17875: f64, t1011: f64, t1019: f64, t3030: f64, t5848: f64, t3032: f64, t3129: f64, t3038: f64, t10891: f64, t17655: f64, t3117: f64, t18029: f64, t3108: f64, t17919: f64, t3070: f64, t42488: f64, t1041: f64, t10868: f64, t248: f64, t5685: f64, t14134: f64, t4644: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61715, t61729, t61731, t61734, t61736, t61739, t61742) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2235(t17884, t3048, t1009, t17875, t1011, t1019, t3030, t5848, t3032, t3129, t3038, t10891, t17655);
        let (t61744, t61754, t61768, t61782, t61784) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2236(t17884, t3117, t18029, t3108, t17919, t3070, t42488, t1041, t10868, t248, t5685, t14134, t4644);
    (t61715, t61729, t61731, t61734, t61736, t61739, t61742, t61744, t61754, t61768, t61782, t61784)
}
