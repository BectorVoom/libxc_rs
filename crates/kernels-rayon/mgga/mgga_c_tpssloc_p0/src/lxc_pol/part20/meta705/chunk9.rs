//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2687/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2687(t16081: f64, t16086: f64, t12214: f64, t67: f64, t792: f64, t16095: f64, t3734: f64, t686: f64, t133: f64, t1799: f64, t40369: f64, t6600: f64) -> (f64, f64, f64) {
    let t54711 = t16081 * t16086;
    let t54718 = t792 * t12214 * t67;
    let t54721 = t54718 * t686 * t16095 * t3734;
    let t54725 = t40369 * t133 * t6600 * t1799;
    (t54711, t54721, t54725)
}
