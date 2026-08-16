//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1353/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1353(t13435: f64, t6993: f64, t10263: f64, t10416: f64, t13216: f64, t2322: f64, t2331: f64, t2372: f64, t25078: f64, t25800: f64, t25805: f64, t25872: f64, t4254: f64, t649: f64, t6985: f64, t7007: f64, t95032: f64, t95036: f64, t95038: f64, t95040: f64, t95042: f64, t95046: f64, t95049: f64, t95056: f64, t95058: f64, t95066: f64, t95068: f64) -> f64 {
    let t95070 = 12.0_f64 * t13435 * t6993;
    let t95071 = -6.0_f64 * t10263 * t6985 - 6.0_f64 * t10416 * t7007 - 6.0_f64 * t13216 * t6985 - 12.0_f64 * t2322 * t25872 - 12.0_f64 * t2331 * t25805 - 6.0_f64 * t2372 * t25805 - 6.0_f64 * t25078 * t4254 - 3.0_f64 * t25800 * t649 - t95032 + t95036 - t95038 - t95040 - t95042 + t95046 - t95049 + t95056 + t95058 - t95066 - t95068 - t95070;
    t95071
}
