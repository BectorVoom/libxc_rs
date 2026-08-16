//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 249/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk249(t871: f64, t875: f64, t296: f64, t193: f64, t446: f64, t834: f64, t837: f64, t842: f64, t865: f64, t89: f64, t312: f64, t863: f64) -> (f64, f64, f64, f64) {
    let t876 = t871 * t875;
    let t877 = t296 * t876;
    let t880 = -t834 - t446 * t837 / 9.0_f64 - t446 * t842 / 3.0_f64 + t89 * t193 * t865 / 3.0_f64 - t446 * t877 / 3.0_f64;
    let t882 = t863 * t312;
    (t876, t877, t880, t882)
}
