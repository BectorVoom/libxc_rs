//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2532/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2532<F: Float>(t43835: F, t43837: F, t43839: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t50881: F, t50886: F, t50897: F, t50900: F) -> F {
    let t51293 = F::cast_from(0.250068e1_f64) * t50881 - F::cast_from(0.104195e0_f64) * t50886 + F::cast_from(0.13892666666666666667e0_f64) * t43835 - F::cast_from(0.41678000000000000001e0_f64) * t43837 - F::cast_from(0.69463333333333333333e-1_f64) * t43839 - F::cast_from(0.11577222222222222222e0_f64) * t43855 - F::cast_from(0.30872592592592592592e-1_f64) * t43857 - F::cast_from(0.92617777777777777776e0_f64) * t43859 + F::cast_from(0.34731666666666666666e0_f64) * t43861 + F::cast_from(0.69463333333333333333e0_f64) * t43863 - F::cast_from(0.34431666666666666667e0_f64) * t50897 - F::cast_from(0.123954e2_f64) * t50900;
    t51293
}
