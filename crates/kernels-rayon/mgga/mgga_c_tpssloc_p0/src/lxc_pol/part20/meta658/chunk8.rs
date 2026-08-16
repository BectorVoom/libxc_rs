//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2448/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2448(t14159: f64, t2960: f64, t1041: f64, t13969: f64, t14146: f64, t10422: f64, t14068: f64, t3070: f64, t10263: f64, t4603: f64, t10891: f64, t13970: f64) -> (f64, f64, f64, f64, f64) {
    let t50077 = t2960 * t14159;
    let t50078 = t50077 / 162.0_f64;
    let t50084 = t1041 * t13969 * t14146;
    let t50094 = t3070 * t10422 * t14068;
    let t50098 = t10263 * t4603;
    let t50100 = t10891 * t13970;
    (t50078, t50084, t50094, t50098, t50100)
}
