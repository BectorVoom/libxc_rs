//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 327/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk327(t1404: f64, t436: f64, t468: f64, t507: f64, t1134: f64, t1179: f64, t1248: f64, t1268: f64, t1273: f64, t1277: f64, t1280: f64, t174: f64, t385: f64, t426: f64, t459: f64, t466: f64, t508: f64, t526: f64, t569: f64) -> f64 {
    let t1405 = t436 * t1404;
    let t1408 = t468 * t507;
    let t1411 = -0.3475929712541504153e-2_f64 * t1134 * t174 - 0.3475929712541504153e-2_f64 * t385 * t508 + 0.10427789137624512459e-2_f64 * t1268 * t174 + 0.20855578275249024918e-2_f64 * t426 * t508 + 0.46345729500553388707e-2_f64 * t1273 * t174 - t1179 + 0.3475929712541504153e-2_f64 * t1277 * t459 + 0.3475929712541504153e-2_f64 * t1280 * t569 - t1248 - 0.10427789137624512459e-2_f64 * t526 * t1405 - 0.6951859425083008306e-4_f64 * t466 * t1408;
    t1411
}
