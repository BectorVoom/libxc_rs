//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta291<F: Float>(t10913: F, t4583: F, t4582: F, t4588: F, t698: F, t999: F, t973: F, t2960: F, t3139: F, t1000: F, t1020: F, t1025: F, t10263: F, t1041: F, t1046: F, t10517: F, t10860: F, t10863: F, t10866: F, t10871: F, t10873: F, t10876: F, t10879: F, t10883: F, t10886: F, t10891: F, t10896: F, t10898: F, t10904: F, t10909: F, t3043: F, t3057: F, t3109: F, t3117: F, t3123: F, t3134: F) -> (F, F, F, F, F, F, F, F) {
        let (t10914, t10915, t10918, t10919, t10922, t10923, t10927, t10929) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1500::<F>(t10913, t4583, t4582, t4588, t698, t999, t973, t2960, t3139, t1000, t1020, t1025, t10263, t1041, t1046, t10517, t10860, t10863, t10866, t10871, t10873, t10876, t10879, t10883, t10886, t10891, t10896, t10898, t10904, t10909, t3043, t3057, t3109, t3117, t3123, t3134);
    (t10914, t10915, t10918, t10919, t10922, t10923, t10927, t10929)
}
