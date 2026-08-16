//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 594/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk594(t236: f64, t8425: f64, t3352: f64, t1970: f64, t1475: f64, t333: f64) -> (f64, f64, f64) {
    let t8426 = t236 * t8425;
    let t8427 = t3352 * t8426;
    let t8428 = t1970 * t8427;
    let t8430 = t1475 * t333;
    (t8427, t8428, t8430)
}
