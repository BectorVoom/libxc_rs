//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 602/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk602(t2929: f64, t315: f64, t323: f64) -> (f64, f64, f64) {
    let t2930 = t315 * t2929;
    let t2931 = t323 * t323;
    let t2932 = 1.0_f64 / t2931;
    (t2930, t2931, t2932)
}
