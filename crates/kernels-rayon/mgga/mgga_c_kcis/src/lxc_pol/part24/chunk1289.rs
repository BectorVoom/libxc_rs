//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1289/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1289(t1009: f64, t6533: f64, t1704: f64, t829: f64, t100264: f64, t101084: f64, t101101: f64, t27812: f64, t2894: f64, t4580: f64, t71184: f64, t7703: f64, t7704: f64, t922: f64, t92730: f64, t93592: f64, t95775: f64, t95779: f64, t95783: f64, t95798: f64, t96108: f64, t9924: f64) -> (f64, f64, f64) {
    let t101111 = t1009 * t6533;
    let t101122 = t1704 * t829;
    let t101127 = 0.37134344353515625001e-4_f64 * t27812 * t101101 - 0.88437037037037037035e-2_f64 * t100264 - 0.92673611111111111112e-3_f64 * t95775 - 0.36848765432098765431e-3_f64 * t92730 - t95779 + t95783 - 0.46336805555555555557e-3_f64 * t7703 * t9924 * t101111 * t922 + 0.46336805555555555556e-3_f64 * t7703 * t2894 * t7704 * t71184 + t95798 - 0.13901041666666666667e-2_f64 * t7703 * t101084 - 0.92673611111111111112e-3_f64 * t93592 * t96108 * t4580 * t101122;
    (t101111, t101122, t101127)
}
