//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1035/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1035(t3351: f64, t3352: f64, t511: f64, t6418: f64, t1704: f64, t503: f64, t681: f64, t1971: f64, t495: f64, t8517: f64, t9969: f64, t41914: f64, t8571: f64) -> (f64, f64, f64, f64) {
    let t47585 = t3351 * t3352 * t511 * t6418;
    let t47587 = t503 * t1704;
    let t47588 = t47587 * t681;
    let t47594 = t8517 * t1971 * t511 * t9969 * t495;
    let t47596 = t8571 * t41914;
    (t47585, t47588, t47594, t47596)
}
