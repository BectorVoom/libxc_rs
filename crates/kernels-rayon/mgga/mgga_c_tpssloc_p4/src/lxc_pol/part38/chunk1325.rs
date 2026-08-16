//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1325/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1325(t29895: f64, t30165: f64, t2331: f64, t2585: f64, t2: f64, t666: f64, t29900: f64, t30172: f64, t110093: f64, t110097: f64, t1444: f64, t1453: f64, t2248: f64, t2332: f64, t2342: f64, t2358: f64, t26129: f64, t29903: f64, t29907: f64, t29922: f64, t29926: f64, t30164: f64, t30171: f64, t30175: f64, t659: f64, t8128: f64, t8137: f64, t8138: f64) -> f64 {
    let t110586 = 20.0_f64 / 9.0_f64 * t29895 * t30165;
    let t110601 = t2585 * t2331;
    let t110602 = t2 * t666;
    let t110615 = 20.0_f64 / 27.0_f64 * t29900 * t30172;
    let t110623 = 5.0_f64 / 12.0_f64 * t8128 * t8138 * t1453 * t2248 - t110586 + 5.0_f64 / 2.0_f64 * t29903 * t29907 * t26129 + 5.0_f64 / 18.0_f64 * t8128 * t29926 * t1453 * t2342 - 5.0_f64 / 4.0_f64 * t29903 * t8138 * t1444 * t2332 - 25.0_f64 / 18.0_f64 * t8128 * t29922 * t30164 + 5.0_f64 / 6.0_f64 * t110601 * t8138 * t110602 + 5.0_f64 / 108.0_f64 * t8137 * t110097 * t1444 * t2342 - 5.0_f64 / 18.0_f64 * t30175 * t29926 * t2 * t659 + t110615 + 5.0_f64 / 12.0_f64 * t8128 * t8138 * t1444 * t2358 + 25.0_f64 / 54.0_f64 * t8137 * t110093 * t30171;
    t110623
}
