//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1243/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1243(t16823: f64, t28351: f64, t4012: f64, t39052: f64, t491: f64, t990: f64, t1928: f64, t3964: f64, t1385: f64, t27370: f64, t3717: f64, t5732: f64) -> (f64, f64, f64, f64) {
    let t98286 = t28351 * t16823 * t4012;
    let t98290 = t39052 * t491 * t990;
    let t98294 = t3964 * t1928 * t990;
    let t98304 = t27370 * t3717 * t5732 * t1385;
    (t98286, t98290, t98294, t98304)
}
