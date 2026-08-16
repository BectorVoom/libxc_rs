//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 565/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk565(t7662: f64, t1173: f64, t2189: f64, t674: f64, t4616: f64, t664: f64) -> (f64, f64, f64, f64) {
    let t7663 = 0.64905642291407286545e-3_f64 * t7662;
    let t7690 = t2189 * t1173;
    let t7691 = t7690 * t674;
    let t7703 = t4616 * t664;
    (t7663, t7690, t7691, t7703)
}
