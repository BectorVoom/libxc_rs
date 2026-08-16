//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1229/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1229(t15874: f64, t4160: f64, t94425: f64, t1464: f64, t1489: f64, t28503: f64, t6037: f64, t1394: f64, t27379: f64, t28499: f64, t27383: f64, t4153: f64) -> (f64, f64, f64, f64) {
    let t98039 = t4160 * t94425 * t15874;
    let t98043 = t1464 * t28503 * t6037 * t1489;
    let t98046 = t1394 * t28499 * t27379;
    let t98049 = t4153 * t28499 * t27383;
    (t98039, t98043, t98046, t98049)
}
