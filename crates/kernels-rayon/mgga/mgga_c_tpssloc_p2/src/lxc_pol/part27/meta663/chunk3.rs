//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2328/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2328(t16217: f64, t6952: f64, t1827: f64, t80910: f64, t22756: f64, t5289: f64, t16208: f64, t6945: f64, t16060: f64, t6951: f64, t1369: f64, t1878: f64, t80730: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91183 = t6952 * t16217;
    let t91185 = t80910 * t1827;
    let t91187 = t22756 * t5289;
    let t91189 = t6945 * t16208;
    let t91191 = t16060 * t6951;
    let t91192 = t91191 * t1369;
    let t91194 = t1878 * t80730;
    (t91183, t91185, t91187, t91189, t91192, t91194)
}
