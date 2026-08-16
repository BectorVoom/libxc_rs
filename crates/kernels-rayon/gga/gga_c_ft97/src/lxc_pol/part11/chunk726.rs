//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 726/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk726(t3885: f64, t9853: f64, t2606: f64, t9723: f64, t9727: f64, t9735: f64, t9701: f64, t9730: f64, t9520: f64, t9695: f64, t9705: f64, t9711: f64, t9715: f64, t9720: f64, t9739: f64, t9752: f64) -> (f64, f64, f64) {
    let t9854 = t3885 * t9853;
    let t9855 = t2606 * t9854;
    let t9861 = t9723 / 9.0_f64;
    let t9862 = 2.0_f64 / 27.0_f64 * t9727;
    let t9863 = 4.0_f64 / 27.0_f64 * t9735;
    let t9867 = 4.0_f64 / 9.0_f64 * t9701;
    let t9869 = 2.0_f64 / 3.0_f64 * t9730;
    let t9870 = t9520 / 3.0_f64;
    let t9871 = -t9705 / 9.0_f64 + 2.0_f64 * t9715 - 10.0_f64 / 81.0_f64 * t9720 + t9861 + t9862 - t9863 - 2.0_f64 / 3.0_f64 * t9739 + 4.0_f64 / 9.0_f64 * t9752 - t9695 / 3.0_f64 - t9867 - 2.0_f64 * t9711 - t9869 + t9870;
    (t9854, t9855, t9871)
}
