//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 82/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk82(t16: f64, t13: f64, t2: f64, t4: f64, t7: f64) -> (f64, f64, f64, f64) {
    let t239 = t16 * t16;
    let t240 = 1.0_f64 / t239;
    let t242 = 1.0_f64 / t13 * t2;
    let t243 = t4 * t7;
    (t239, t240, t242, t243)
}
