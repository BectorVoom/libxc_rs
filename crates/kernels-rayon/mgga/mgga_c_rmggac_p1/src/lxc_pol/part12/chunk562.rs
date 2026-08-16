//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 562/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk562(t7472: f64, t7473: f64, t128: f64, t209: f64, t476: f64, t118: f64, t1986: f64) -> (f64, f64) {
    let t7474 = t7472 * t7473;
    let t7476 = t128 * t476 * t209;
    let t7477 = t118 * t7476;
    let t7478 = t1986 * t7477;
    (t7474, t7478)
}
