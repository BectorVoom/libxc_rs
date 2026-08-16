//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 525/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk525(t6567: f64, t6616: f64, t1810: f64, t941: f64, t1664: f64, t574: f64, t271: f64, t830: f64) -> (f64, f64, f64, f64) {
    let t6617 = t6567 + t6616;
    let t6624 = t941 * t1810;
    let t6627 = t1664 * t574;
    let t7184 = t830 * t271;
    (t6617, t6624, t6627, t7184)
}
