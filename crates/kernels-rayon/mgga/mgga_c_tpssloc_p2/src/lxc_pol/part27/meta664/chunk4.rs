//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2334/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2334(t16314: f64, t26309: f64, t16227: f64, t22833: f64, t1339: f64, t57643: f64, t6936: f64, t22827: f64, t550: f64, t56805: f64, t54165: f64, t16060: f64, t6944: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91261 = t26309 * t16314;
    let t91263 = t22833 * t16227;
    let t91268 = t6936 * t1339 * t57643;
    let t91272 = t22827 * t1339 * t56805 * t550;
    let t91276 = t22827 * t1339 * t54165 * t550;
    let t91278 = t16060 * t6944;
    (t91261, t91263, t91268, t91272, t91276, t91278)
}
