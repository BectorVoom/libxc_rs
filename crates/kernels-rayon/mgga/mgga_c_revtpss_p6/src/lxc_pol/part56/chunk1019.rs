//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1019/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1019(t4147: f64, t8594: f64, t8598: f64, t9593: f64, t11239: f64, t13181: f64, t3736: f64, t1450: f64, t211: f64, t9644: f64, t11006: f64, t256: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36970 = t4147 * t8594;
    let t37110 = t9593 * t8598;
    let t37880 = t11239 * t13181;
    let t37885 = t11239 * t3736;
    let t37956 = t8594 * t1450;
    let t37972 = t8598 * t4147;
    let t39643 = 1.0_f64 / t9644 / t211;
    let t41077 = 1.0_f64 / t11006 / t256;
    (t36970, t37110, t37880, t37885, t37956, t37972, t39643, t41077)
}
