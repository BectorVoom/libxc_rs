//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1193/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1193(t11064: f64, t7427: f64, t1940: f64, t2071: f64, t2072: f64, t2257: f64, t2403: f64, t25211: f64, t25215: f64, t25446: f64, t25452: f64, t26425: f64, t26581: f64, t26585: f64, t26590: f64, t28472: f64, t4541: f64, t605: f64, t7428: f64, t92747: f64, t92762: f64, t92783: f64, t92795: f64, t92799: f64, t92806: f64, t92814: f64, t92822: f64, t9344: f64) -> (f64, f64) {
    let t95976 = t7427 * t11064;
    let t96016 = 9.0_f64 * t4541 * t2071 * t92806 + 3.0_f64 * t1940 * t95976 * t25446 + 3.0_f64 * t92822 * t2072 + 3.0_f64 * t28472 * t92762 + 3.0_f64 * t1940 * t26590 * t92783 + 9.0_f64 * t2403 * t7428 * t25211 + 9.0_f64 / 2.0_f64 * t2403 * t7428 * t25215 - 3.0_f64 / 2.0_f64 * t1940 * t26585 * t25452 + t1940 * t2071 * t9344 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t1940 * t26581 * t605 + 9.0_f64 * t26425 * t92747 + 9.0_f64 / 2.0_f64 * t2403 * t2071 * t92795 + 9.0_f64 / 2.0_f64 * t2403 * t2071 * t92799 + 3.0_f64 / 2.0_f64 * t1940 * t7428 * t2257 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t92814;
    (t95976, t96016)
}
