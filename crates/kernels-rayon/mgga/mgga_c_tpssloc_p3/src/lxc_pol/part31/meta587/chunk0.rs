//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1829/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1829(t26322: f64, t80855: f64, t91152: f64, t236: f64, t26318: f64, t91005: f64, t22782: f64, t5234: f64, t1369: f64, t7712: f64, t80939: f64, t22683: f64, t26285: f64, t6546: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91154 = t91152 * t80855 * t26322;
    let t91158 = t91152 * t91005 * t236 * t26318;
    let t91160 = t5234 * t22782;
    let t91161 = t91160 * t1369;
    let t91167 = t80939 * t7712;
    let t91170 = t6546 * t22683 * t26285;
    (t91154, t91158, t91160, t91161, t91167, t91170)
}
