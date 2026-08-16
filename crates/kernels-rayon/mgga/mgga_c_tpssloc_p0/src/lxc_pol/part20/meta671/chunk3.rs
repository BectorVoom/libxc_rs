//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2524/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2524(t43835: f64, t43837: f64, t43839: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t50881: f64, t50886: f64, t50897: f64, t50900: f64) -> f64 {
    let t51173 = 0.197176e1_f64 * t50881 - 0.82156666666666666668e-1_f64 * t50886 + 0.10954222222222222222e0_f64 * t43835 - 0.32862666666666666666e0_f64 * t43837 - 0.54771111111111111111e-1_f64 * t43839 - 0.91285185185185185185e-1_f64 * t43855 - 0.24342716049382716049e-1_f64 * t43857 - 0.73028148148148148149e0_f64 * t43859 + 0.27385555555555555556e0_f64 * t43861 + 0.54771111111111111111e0_f64 * t43863 - 0.19931111111111111111e0_f64 * t50897 - 0.71752000000000000002e1_f64 * t50900;
    t51173
}
