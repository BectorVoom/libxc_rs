//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2458/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2458(t22715: f64, t268: f64, t405: f64, t2403: f64, t3298: f64, t1114: f64, t9709: f64, t3304: f64, t3301: f64, t39267: f64, t404: f64, t410: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43819 = t268 * t22715 * t405;
    let t43820 = 280.0_f64 / 81.0_f64 * t43819;
    let t43855 = t2403 * t3298;
    let t43859 = t9709 * t1114;
    let t43861 = t2403 * t3304;
    let t43863 = t2403 * t3301;
    let t43880 = 1.0_f64 / t410 / t39267 / t404 / 96.0_f64;
    (t43819, t43820, t43855, t43859, t43861, t43863, t43880)
}
