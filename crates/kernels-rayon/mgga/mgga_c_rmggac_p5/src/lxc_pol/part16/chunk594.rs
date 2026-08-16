//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 594/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk594(t511: f64, t8430: f64, t1971: f64, t1970: f64, t1475: f64, t352: f64) -> (f64, f64, f64) {
    let t8431 = t511 * t8430;
    let t8432 = t1971 * t8431;
    let t8433 = t1970 * t8432;
    let t8435 = t1475 * t352;
    (t8432, t8433, t8435)
}
