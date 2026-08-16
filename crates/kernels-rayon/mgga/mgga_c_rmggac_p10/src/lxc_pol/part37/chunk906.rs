//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 906/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk906(t27101: f64, t75845: f64, t35960: f64, t649: f64, t8985: f64, t40928: f64, t8976: f64, t8947: f64, t11704: f64, t14293: f64, t14296: f64, t1652: f64, t27: f64, t29: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76253 = 0.11974241701863808564e0_f64 * t27101 * t75845;
    let t76255 = t35960 * t649 * t8985;
    let t76258 = t40928 * t649 * t8976;
    let t76262 = t35960 * t649 * t8947;
    let t76264 = t14293 * t11704;
    let t76268 = t14296 * t27 * t29 * t1652;
    (t76253, t76255, t76258, t76262, t76264, t76268)
}
