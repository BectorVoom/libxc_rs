//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2338/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2338(t22788: f64, t5310: f64, t16150: f64, t6952: f64, t16155: f64, t26271: f64, t80836: f64, t1361: f64, t22690: f64, t22792: f64, t5187: f64, t16148: f64, t26288: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91317 = t22788 * t5310;
    let t91319 = t6952 * t16150;
    let t91321 = t6952 * t16155;
    let t91323 = t80836 * t26271;
    let t91327 = t22792 * t22690 * t1361 * t5187;
    let t91328 = 0.40372756094140390854e-3_f64 * t91327;
    let t91330 = t26288 * t1361 * t16148;
    (t91317, t91319, t91321, t91323, t91328, t91330)
}
