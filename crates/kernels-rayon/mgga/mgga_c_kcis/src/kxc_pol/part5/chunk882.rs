//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 882/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk882(t577: f64, t7321: f64, t585: f64, t1926: f64, t488: f64, t579: f64, t251: f64, t584: f64, t578: f64, t2061: f64, t2065: f64, t2038: f64, t2042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7322 = t7321 * t577;
    let t7323 = t7322 * t585;
    let t7327 = 1.0_f64 / t488 / t579 / t1926;
    let t7328 = t7327 * t251;
    let t7329 = t7328 * t584;
    let t7330 = t578 * t7329;
    let t7332 = t2061 * t2065;
    let t7333 = t578 * t7332;
    let t7335 = t2042 * t2038;
    (t7322, t7323, t7328, t7329, t7330, t7332, t7333, t7335)
}
