//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 640/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk640(t342: f64, t4645: f64, t630: f64, t1882: f64, t4657: f64, t4668: f64, t7368: f64, t1546: f64, t4664: f64, t89: f64, t4660: f64, t4652: f64, t7780: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16649 = t342 * t630 * t4645;
    let t16679 = t1882 * t4657;
    let t16736 = t7368 * t4668;
    let t16745 = t89 * t1546 * t4664;
    let t16748 = t89 * t1546 * t4660;
    let t16751 = t89 * t7780 * t4652;
    (t16649, t16679, t16736, t16745, t16748, t16751)
}
