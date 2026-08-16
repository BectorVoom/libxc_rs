//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1324/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1324(t110075: f64, t30149: f64, t29895: f64, t30156: f64, t110082: f64, t110148: f64, t110150: f64, t1444: f64, t2176: f64, t2248: f64, t2585: f64, t29903: f64, t29922: f64, t29926: f64, t30155: f64, t4067: f64, t659: f64, t8128: f64, t8129: f64, t8137: f64, t8138: f64, t86592: f64, t86595: f64, t86598: f64, t95: f64) -> f64 {
    let t110564 = 4.0_f64 * t110075 * t30149;
    let t110566 = 20.0_f64 / 9.0_f64 * t29895 * t30156;
    let t110580 = -50.0_f64 / 27.0_f64 * t110148 + 5.0_f64 / 9.0_f64 * t110150 - 5.0_f64 / 24.0_f64 * t2585 * t2176 * t95 - 5.0_f64 / 36.0_f64 * t8137 * t29926 * t1444 * t2248 + 3.0_f64 * t110082 * t8129 * t86592 + t110564 - t110566 - 3.0_f64 / 2.0_f64 * t29903 * t8129 * t86595 - 3.0_f64 / 4.0_f64 * t29903 * t8129 * t86598 - 25.0_f64 / 18.0_f64 * t8128 * t29922 * t30155 + 5.0_f64 / 6.0_f64 * t8128 * t8138 * t4067 * t659;
    t110580
}
