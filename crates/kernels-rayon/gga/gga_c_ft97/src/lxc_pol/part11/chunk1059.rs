//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1059/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1059(t193: f64, t2373: f64, t2459: f64, t7514: f64, t89: f64, t3704: f64, t670: f64, t41932: f64, t41935: f64, t41938: f64, t41942: f64, t41947: f64, t41951: f64, t41954: f64, t41958: f64, t41960: f64, t41964: f64, t41969: f64, t41973: f64) -> (f64, f64, f64) {
    let t41978 = t89 * t193 * t7514 * t2373 * t2459;
    let t41981 = t89 * t3704 * t670;
    let t41982 = 56.0_f64 / 243.0_f64 * t41981;
    let t41983 = 2.0_f64 / 9.0_f64 * t41932 + 4.0_f64 / 9.0_f64 * t41935 - 4.0_f64 / 27.0_f64 * t41938 + 4.0_f64 / 3.0_f64 * t41942 + 2.0_f64 / 9.0_f64 * t41947 + t41951 - t41954 - t41958 - 8.0_f64 / 27.0_f64 * t41960 + 20.0_f64 / 243.0_f64 * t41964 + 20.0_f64 / 27.0_f64 * t41969 - t41973 / 18.0_f64 - 6.0_f64 * t41978 + t41982;
    (t41978, t41981, t41983)
}
