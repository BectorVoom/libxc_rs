//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 584/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk584(t7780: f64, t1347: f64, t703: f64, t2244: f64, t275: f64, t7908: f64, t7910: f64, t7818: f64, t7820: f64, t2227: f64, t874: f64, t7937: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8197 = 0.15965655602485078085e0_f64 * t7780;
    let t8201 = t1347 * t703;
    let t8208 = t275 * t2244;
    let t8209 = 2.0_f64 * t8208;
    let t8221 = 0.39726959900411316772e-4_f64 * t7908;
    let t8222 = 0.11918087970123395032e-3_f64 * t7910;
    let t8242 = 0.2927036860455597649e0_f64 * t7818;
    let t8243 = 0.66671395154821946452e-1_f64 * t7820;
    let t8264 = t874 * t2227;
    let t8303 = 0.1440846329149835838e-2_f64 * t7937;
    (t8197, t8201, t8209, t8221, t8222, t8242, t8243, t8264, t8303)
}
