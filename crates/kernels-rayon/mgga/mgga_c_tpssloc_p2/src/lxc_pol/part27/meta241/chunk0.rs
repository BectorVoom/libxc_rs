//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1156/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1156(t1894: f64, t236: f64, t776: f64, t6591: f64, t2229: f64, t61: f64) -> (f64, f64, f64) {
    let t6593 = t1894 * t236 * t776;
    let t6594 = t6591 * t6593;
    let t6597 = 1.0_f64 / t61 / t2229;
    (t6593, t6594, t6597)
}
