//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 947/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk947(t11875: f64, t4098: f64, t673: f64, t4095: f64, t1502: f64, t2193: f64) -> (f64, f64, f64, f64, f64) {
    let t11876 = 0.39862222222222222222e0_f64 * t11875;
    let t11910 = t673 * t4098;
    let t11911 = 0.21908444444444444444e0_f64 * t11910;
    let t11932 = t673 * t4095;
    let t11938 = t2193 * t1502;
    (t11876, t11910, t11911, t11932, t11938)
}
