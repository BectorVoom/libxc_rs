//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1858/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1858(t3196: f64, t6800: f64, t6799: f64, t23602: f64, t3127: f64, t1011: f64, t3131: f64) -> (f64, f64, f64, f64) {
    let t23673 = t3196 * t6800;
    let t23674 = t6799 * t23673;
    let t23677 = t23602 * t3127;
    let t23678 = t1011 * t3131;
    (t23673, t23674, t23677, t23678)
}
