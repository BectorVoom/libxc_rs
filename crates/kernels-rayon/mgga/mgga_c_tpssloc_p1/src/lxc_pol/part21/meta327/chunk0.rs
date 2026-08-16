//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1697/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1697(t1307: f64, t212: f64, t12225: f64, t2586: f64, t535: f64, t9534: f64, t9538: f64, t3792: f64, t3850: f64) -> (f64, f64, f64, f64) {
    let t12226 = t212 * t1307;
    let t12227 = t12225 * t12226;
    let t12228 = t2586 * t12227;
    let t12236 = 0.13888888888888888889e-3_f64 * t9534 * t535 * t9538;
    let t12240 = t3792 * t3850;
    (t12227, t12228, t12236, t12240)
}
