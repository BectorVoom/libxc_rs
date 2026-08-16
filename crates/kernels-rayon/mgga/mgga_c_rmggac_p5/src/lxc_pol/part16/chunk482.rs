//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 482/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk482(t201: f64, t6070: f64, t1856: f64, t457: f64, t1451: f64, t228: f64, t1859: f64, t5542: f64, t615: f64, t1173: f64, t1864: f64, t495: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6071 = t6070 * t201;
    let t6073 = t1856 * t457;
    let t6077 = t228 * t1451;
    let t6080 = t1859 * t457;
    let t6086 = t5542 * t615;
    let t6093 = t1173 * t1864;
    let t6096 = t1864 * t495;
    (t6071, t6073, t6077, t6080, t6086, t6093, t6096)
}
