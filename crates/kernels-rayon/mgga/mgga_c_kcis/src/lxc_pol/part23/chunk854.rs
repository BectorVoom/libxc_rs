//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 854/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk854(t16144: f64, t5564: f64, t659: f64, t16050: f64, t11409: f64, t11411: f64, t11413: f64, t11415: f64, t11455: f64, t11457: f64, t11460: f64, t16048: f64, t16062: f64, t16088: f64) -> (f64, f64, f64) {
    let t16145 = 0.21908444444444444444e0_f64 * t16144;
    let t16146 = t659 * t5564;
    let t16156 = 0.39862222222222222222e0_f64 * t16050;
    let t16160 = -0.26574814814814814816e0_f64 * t11409 + 0.66437037037037037038e-1_f64 * t11411 - 0.19931111111111111111e0_f64 * t11413 + 0.99655555555555555557e-1_f64 * t11415 + 0.59793333333333333334e0_f64 * t16088 + 0.11958666666666666667e1_f64 * t16062 + 0.13287407407407407408e0_f64 * t16048 - t16156 - 0.18257037037037037037e0_f64 * t11455 + 0.54771111111111111111e-1_f64 * t11457 + 0.18257037037037037037e-1_f64 * t11460;
    (t16145, t16146, t16160)
}
