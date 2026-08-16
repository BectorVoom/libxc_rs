//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1180/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1180(t13791: f64, t1429: f64, t549: f64, t41734: f64, t41735: f64, t41736: f64, t41737: f64, t41741: f64, t41743: f64, t41744: f64, t41747: f64, t41752: f64, t41753: f64, t41754: f64) -> f64 {
    let t47892 = t1429 * t549 * t13791;
    let t47894 = -t41734 - t41735 - t41736 + t41737 + 0.29792074959875355558e-1_f64 * t47892 + t41741 + t41743 - t41744 + t41747 - t41752 - t41753 + t41754;
    t47894
}
