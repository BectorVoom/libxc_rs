//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 874/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk874(t1594: f64, t1986: f64, t7720: f64, t1627: f64, t3352: f64, t495: f64, t511: f64, t7230: f64, t2410: f64, t7228: f64, t1969: f64, t7457: f64) -> (f64, f64, f64, f64) {
    let t39199 = t1986 * t1594;
    let t39200 = t7720 * t39199;
    let t39205 = t7230 * t3352 * t511 * t1627 * t495;
    let t39207 = t2410 * t7228;
    let t39208 = t39207 * t1969;
    let t39209 = t39208 * t7457;
    (t39200, t39205, t39207, t39209)
}
