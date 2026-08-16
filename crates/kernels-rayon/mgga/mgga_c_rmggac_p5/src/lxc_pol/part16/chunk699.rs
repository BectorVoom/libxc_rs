//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 699/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk699(t10088: f64, t511: f64, t7231: f64, t3351: f64, t570: f64, t618: f64) -> (f64, f64, f64) {
    let t10089 = t511 * t10088;
    let t10090 = t7231 * t10089;
    let t10091 = t3351 * t10090;
    let t10093 = t618 * t570;
    (t10090, t10091, t10093)
}
