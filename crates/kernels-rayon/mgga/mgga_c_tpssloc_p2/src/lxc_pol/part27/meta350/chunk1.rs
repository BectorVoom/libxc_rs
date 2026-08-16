//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1458/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1458(t12997: f64, t792: f64, t12984: f64, t686: f64, t776: f64, t12986: f64, t12990: f64, t12994: f64, t4127: f64, t9526: f64, t9540: f64, t9542: f64, t9544: f64, t9547: f64, t9552: f64, t9556: f64) -> f64 {
    let t12998 = t792 * t12997;
    let t13000 = t686 * t12984 * t776;
    let t13002 = 0.49999999999999999998e-2_f64 * t12998 * t13000;
    let t13003 = 0.33333333333333333332e-2_f64 * t9526 - t9540 - 0.25925925925925925926e-1_f64 * t9542 + 0.38888888888888888888e-2_f64 * t9544 - 0.10555555555555555555e-1_f64 * t9547 - 0.25e-2_f64 * t9552 + 0.83333333333333333332e-3_f64 * t9556 + 0.16666666666666666666e-2_f64 * t12986 + 0.99999999999999999996e-2_f64 * t4127 * t12990 + 0.49999999999999999998e-2_f64 * t4127 * t12994 - t13002;
    t13003
}
