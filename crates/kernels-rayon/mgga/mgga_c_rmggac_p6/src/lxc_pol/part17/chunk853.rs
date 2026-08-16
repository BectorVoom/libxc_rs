//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 853/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk853(t42177: f64, t1982: f64, t7428: f64, t8608: f64, t2139: f64, t27: f64, t3118: f64, t558: f64, t40975: f64, t7192: f64, t16156: f64, t9194: f64) -> (f64, f64, f64, f64, f64) {
    let t42178 = 0.19863479950205658386e-4_f64 * t42177;
    let t42180 = t8608 * t7428 * t1982;
    let t42181 = 0.19863479950205658386e-4_f64 * t42180;
    let t42196 = t2139 * t27 * t3118 * t558;
    let t42201 = t7192 * t40975;
    let t42204 = t16156 * t9194;
    (t42178, t42181, t42196, t42201, t42204)
}
