//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1266/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1266(t16610: f64, t303: f64, t7931: f64, t1394: f64, t27427: f64, t28356: f64, t1464: f64, t2046: f64, t28503: f64, t4124: f64, t4136: f64, t51613: f64, t7923: f64) -> (f64, f64, f64, f64) {
    let t98673 = t303 * t7931 * t16610;
    let t98676 = t1394 * t28356 * t27427;
    let t98680 = t1464 * t28503 * t2046 * t4124;
    let t98684 = t1464 * t7923 * t51613 * t4136;
    (t98673, t98676, t98680, t98684)
}
