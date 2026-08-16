//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1192/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1192(t95574: f64, t95622: f64, t95729: f64, t95776: f64, t95821: f64, t95863: f64, t95904: f64, t95950: f64, t892: f64, t2070: f64, t41154: f64, t1940: f64, t2403: f64, t25198: f64, t25208: f64, t25449: f64, t26425: f64, t26581: f64, t26585: f64, t28291: f64, t30: f64, t4541: f64, t7010: f64, t7092: f64, t7428: f64, t7432: f64, t92743: f64, t92753: f64, t92759: f64, t92765: f64, t92768: f64, t92772: f64, t92779: f64, t92791: f64, t92810: f64, t95511: f64, t95527: f64) -> (f64, f64, f64, f64) {
    let t95953 = t95574 + t95622 + t95729 + t95776 + t95821 + t95863 + t95904 + t95950;
    let t95954 = t95953 * t892;
    let t95964 = t2070 * t41154;
    let t95972 = -9.0_f64 * t95511 * t25208 - 9.0_f64 / 2.0_f64 * t26425 * t92765 - 9.0_f64 * t26425 * t92791 - 3.0_f64 / 2.0_f64 * t1940 * t7432 * t92779 - 3.0_f64 / 2.0_f64 * t1940 * t7432 * t92768 - 3.0_f64 * t1940 * t26585 * t25449 - 3.0_f64 / 2.0_f64 * t1940 * t95527 * t7092 - 9.0_f64 / 2.0_f64 * t26425 * t92759 + 9.0_f64 * t4541 * t7428 * t25198 + t1940 * t95954 * t30 / 2.0_f64 + 9.0_f64 / 2.0_f64 * t2403 * t26581 * t7010 - t1940 * t7432 * t92810 / 2.0_f64 - 3.0_f64 * t1940 * t95964 * t92743 - 9.0_f64 * t28291 * t92753 + 9.0_f64 * t28291 * t92772;
    (t95953, t95954, t95964, t95972)
}
