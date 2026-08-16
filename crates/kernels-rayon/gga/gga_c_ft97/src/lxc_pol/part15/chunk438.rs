//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 438/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk438(t378: f64, t4462: f64, t92: f64, t1639: f64, t3042: f64, t4456: f64, t4460: f64) -> (f64, f64, f64) {
    let t4463 = t378 * t4462;
    let t4464 = t92 * t4463;
    let t4466 = t1639 + 2.0_f64 / 9.0_f64 * t3042 - 2.0_f64 / 9.0_f64 * t4456 + 2.0_f64 / 3.0_f64 * t4460 - t4464 / 3.0_f64;
    (t4463, t4464, t4466)
}
