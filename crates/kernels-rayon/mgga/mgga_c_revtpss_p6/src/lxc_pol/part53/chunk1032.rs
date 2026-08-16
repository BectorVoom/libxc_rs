//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1032/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1032(t7313: f64, t8568: f64, t32171: f64, t508: f64, t1310: f64, t8454: f64, t2042: f64, t7324: f64, t2040: f64, t7331: f64, t7334: f64, t1459: f64, t8611: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32329 = t8568 * t7313;
    let t32338 = 2.0_f64 * t32171 * t508;
    let t32340 = 2.0_f64 * t8454 * t1310;
    let t32358 = t7324 * t2042;
    let t32360 = t2040 * t7331;
    let t32362 = t2040 * t7334;
    let t32365 = 6.0_f64 * t1459 * t8611;
    (t32329, t32338, t32340, t32358, t32360, t32362, t32365)
}
