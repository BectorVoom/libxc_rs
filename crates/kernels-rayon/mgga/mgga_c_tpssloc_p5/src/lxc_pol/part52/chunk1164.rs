//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1164/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1164(t31058: f64, t652: f64, t2314: f64, t8323: f64, t4034: f64, t1873: f64, t6862: f64, t6517: f64, t6535: f64, t8526: f64, t1874: f64, t22461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31059 = t652 * t31058;
    let t31060 = 2.0_f64 * t31059;
    let t31065 = t2314 * t8323;
    let t31067 = t4034 * t8323;
    let t31069 = t6862 * t1873;
    let t31070 = t652 * t31069;
    let t31072 = t6517 * t6535;
    let t31077 = 4.0_f64 * t8526 * t6535;
    let t31078 = t22461 * t1874;
    (t31060, t31065, t31067, t31069, t31070, t31072, t31077, t31078)
}
