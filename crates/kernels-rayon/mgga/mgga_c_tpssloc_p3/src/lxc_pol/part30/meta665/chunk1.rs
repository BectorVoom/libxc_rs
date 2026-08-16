//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2091/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2091(t91158: f64, t22782: f64, t5234: f64, t1369: f64, t7712: f64, t80939: f64, t22683: f64, t26285: f64, t6546: f64, t26289: f64, t6604: f64, t80887: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91159 = 0.13457585364713463618e-3_f64 * t91158;
    let t91160 = t5234 * t22782;
    let t91161 = t91160 * t1369;
    let t91162 = 7.0_f64 / 288.0_f64 * t91161;
    let t91167 = t80939 * t7712;
    let t91170 = t6546 * t22683 * t26285;
    let t91171 = 7.0_f64 / 24.0_f64 * t91170;
    let t91179 = t80887 * t6604 * t26289;
    (t91159, t91160, t91162, t91167, t91171, t91179)
}
