//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2069/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2069(t1049: f64, t225: f64, t344: f64, t23384: f64, t23729: f64, t10189: f64, t1926: f64, t221: f64) -> (f64, f64, f64) {
    let t82417 = t344 * t1049 * t225;
    let t82426 = t23384 * t23729;
    let t82431 = t1926 * t221 * t10189;
    (t82417, t82426, t82431)
}
