//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 766/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk766(t23237: f64, t6555: f64, t6552: f64, t2379: f64, t6554: f64, t6553: f64, t23035: f64, t6547: f64, t6568: f64, t23030: f64, t6563: f64, t6567: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23238 = t23237 * t6555;
    let t23239 = t6552 * t23238;
    let t23241 = t6554 * t2379;
    let t23242 = t6553 * t23241;
    let t23243 = t23035 * t23242;
    let t23249 = t6547 * t6568;
    let t23250 = 0.38381794893125283518e-1_f64 * t23249;
    let t23251 = t23030 * t6563;
    let t23252 = 0.26044789391763585244e-1_f64 * t23251;
    let t23253 = t794 * t6567;
    (t23239, t23241, t23243, t23249, t23250, t23251, t23252, t23253)
}
