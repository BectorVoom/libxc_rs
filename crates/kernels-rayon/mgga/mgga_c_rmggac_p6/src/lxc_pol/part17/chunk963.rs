//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 963/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk963(t2283: f64, t9087: f64, t1734: f64, t1970: f64, t209: f64, t236: f64, t3352: f64, t476: f64, t8577: f64, t9159: f64, t1743: f64, t1971: f64, t511: f64) -> (f64, f64, f64, f64) {
    let t45966 = t9087 * t2283;
    let t45974 = t1970 * t3352 * t236 * t1734 * t476 * t209;
    let t45976 = t8577 * t9159;
    let t45982 = t1970 * t1971 * t511 * t1743 * t476 * t209;
    (t45966, t45974, t45976, t45982)
}
