//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 357/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk357(t974: f64, t986: f64, t346: f64, t964: f64, t971: f64, t973: f64, t980: f64) -> (f64, f64) {
    let t987 = t974 * t986;
    let t990 = -0.22222222222222222222e-2_f64 * t964 * t346 + t971 + 0.27777777777777777777e-3_f64 * t973 * t980 - 0.83333333333333333332e-3_f64 * t973 * t987;
    (t987, t990)
}
