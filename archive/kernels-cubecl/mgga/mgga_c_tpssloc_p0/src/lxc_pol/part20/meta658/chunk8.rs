//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2448/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2448<F: Float>(t14159: F, t2960: F, t1041: F, t13969: F, t14146: F, t10422: F, t14068: F, t3070: F, t10263: F, t4603: F, t10891: F, t13970: F) -> (F, F, F, F, F) {
    let t50077 = t2960 * t14159;
    let t50078 = t50077 / F::cast_from(162.0_f64);
    let t50084 = t1041 * t13969 * t14146;
    let t50094 = t3070 * t10422 * t14068;
    let t50098 = t10263 * t4603;
    let t50100 = t10891 * t13970;
    (t50078, t50084, t50094, t50098, t50100)
}
