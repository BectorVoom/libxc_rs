//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2235;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta676<F: Float>(t17884: F, t3048: F, t1009: F, t17875: F, t1011: F, t1019: F, t3030: F, t5848: F, t3032: F, t3129: F, t3038: F, t10891: F, t17655: F, t3117: F, t18029: F, t3108: F, t17919: F, t3070: F, t42488: F, t1041: F, t10868: F, t248: F, t5685: F, t14134: F, t4644: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t61715, t61729, t61731, t61734, t61736, t61739, t61742) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2235::<F>(t17884, t3048, t1009, t17875, t1011, t1019, t3030, t5848, t3032, t3129, t3038, t10891, t17655);
        let (t61744, t61754, t61768, t61782, t61784) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2236::<F>(t17884, t3117, t18029, t3108, t17919, t3070, t42488, t1041, t10868, t248, t5685, t14134, t4644);
    (t61715, t61729, t61731, t61734, t61736, t61739, t61742, t61744, t61754, t61768, t61782, t61784)
}
