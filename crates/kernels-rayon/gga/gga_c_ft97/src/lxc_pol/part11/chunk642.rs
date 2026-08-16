//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 642/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk642(t120: f64, t7899: f64, t378: f64, t8030: f64, t72: f64, t341: f64, t630: f64, t343: f64, t70: f64) -> (f64, f64, f64, f64) {
    let t8949 = t7899 * t120;
    let t8950 = t378 * t8949;
    let t8955 = t8030 * t120;
    let t8956 = t72 * t8955;
    let t8959 = t341 * t630;
    let t8963 = t341 * t343 * t70;
    (t8950, t8956, t8959, t8963)
}
