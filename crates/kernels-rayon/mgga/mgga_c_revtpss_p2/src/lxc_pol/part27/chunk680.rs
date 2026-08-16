//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 680/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk680(t241: f64, t2719: f64, t820: f64, t243: f64, t72: f64, t245: f64, t2723: f64, t836: f64, t162: f64, t2611: f64, t227: f64, t73: f64) -> (f64, f64, f64, f64, f64) {
    let t4362 = t820 * t2719 * t241;
    let t4363 = t243 * t72;
    let t4364 = t4363 * t245;
    let t4366 = t2723 * t836;
    let t4401 = t2611 * t162;
    let t4415 = t227 * t73;
    (t4362, t4364, t4366, t4401, t4415)
}
