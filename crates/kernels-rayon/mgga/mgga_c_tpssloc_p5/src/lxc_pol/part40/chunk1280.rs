//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1280/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1280(t109: f64, t5464: f64, t8129: f64, t1444: f64, t1453: f64, t8138: f64, t5488: f64, t29926: f64, t5468: f64, t5396: f64, t29894: f64, t29903: f64, t30147: f64, t30162: f64, t8128: f64, t8137: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t30407 = t8129 * t5464;
    let t30410 = t1453 * t1444;
    let t30411 = t8138 * t30410;
    let t30414 = t8129 * t5488;
    let t30417 = t29926 * t5468;
    let t30420 = t8138 * t5396;
    let t30424 = piecewise3(t110, 0.0_f64, -t29894 - 4.0_f64 / 3.0_f64 * t30147 + 10.0_f64 / 9.0_f64 * t30162 - 3.0_f64 / 4.0_f64 * t29903 * t30407 + 5.0_f64 / 6.0_f64 * t8128 * t30411 + t8128 * t30414 / 4.0_f64 - 5.0_f64 / 36.0_f64 * t8137 * t30417 - 5.0_f64 / 24.0_f64 * t8137 * t30420);
    (t30407, t30410, t30411, t30414, t30417, t30420, t30424)
}
