//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2532/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2532(t43835: f64, t43837: f64, t43839: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t50881: f64, t50886: f64, t50897: f64, t50900: f64) -> f64 {
    let t51293 = 0.250068e1_f64 * t50881 - 0.104195e0_f64 * t50886 + 0.13892666666666666667e0_f64 * t43835 - 0.41678000000000000001e0_f64 * t43837 - 0.69463333333333333333e-1_f64 * t43839 - 0.11577222222222222222e0_f64 * t43855 - 0.30872592592592592592e-1_f64 * t43857 - 0.92617777777777777776e0_f64 * t43859 + 0.34731666666666666666e0_f64 * t43861 + 0.69463333333333333333e0_f64 * t43863 - 0.34431666666666666667e0_f64 * t50897 - 0.123954e2_f64 * t50900;
    t51293
}
