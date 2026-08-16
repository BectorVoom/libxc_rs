//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 951/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk951(t7440: f64, t761: f64, t33773: f64, t8392: f64, t668: f64, t7553: f64, t33693: f64, t33697: f64, t33776: f64, t33709: f64, t33712: f64, t1882: f64, t33743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t141713 = t761 * t7440;
    let t141722 = t8392 * t33773;
    let t141727 = t7553 * t668;
    let t141744 = t8392 * t33693;
    let t141746 = t8392 * t33697;
    let t141752 = t8392 * t33776;
    let t141759 = t8392 * t33709;
    let t141784 = t8392 * t33712;
    let t141815 = t1882 * t33743;
    (t141713, t141722, t141727, t141744, t141746, t141752, t141759, t141784, t141815)
}
