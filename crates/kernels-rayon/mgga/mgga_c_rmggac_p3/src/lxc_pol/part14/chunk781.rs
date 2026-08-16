//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 781/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk781(t36330: f64, t2131: f64, t4036: f64, t3981: f64, t1969: f64, t8516: f64, t7229: f64, t7243: f64, t7457: f64, t2186: f64, t7424: f64, t7404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36331 = 0.91462949374725084942e-3_f64 * t36330;
    let t36332 = t4036 * t2131;
    let t36334 = t3981 * t2131;
    let t36336 = t8516 * t1969;
    let t36343 = t7229 * t7243;
    let t36344 = t36343 * t7457;
    let t36379 = t2186 * t7424;
    let t36381 = t2186 * t7404;
    (t36331, t36332, t36334, t36336, t36343, t36344, t36379, t36381)
}
