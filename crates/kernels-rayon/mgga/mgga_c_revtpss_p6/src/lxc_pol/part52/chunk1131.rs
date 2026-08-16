//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1131/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1131(t1389: f64, t246: f64, t32247: f64, t32275: f64, t1381: f64, t8590: f64, t94801: f64, t3140: f64, t9656: f64, t1385: f64, t1404: f64, t32276: f64, t32278: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121019 = t1389 * t246;
    let t121024 = t32247 * t32275;
    let t121028 = t94801 * t8590 * t1381;
    let t121029 = 0.3718732920905101082e-4_f64 * t121028;
    let t121034 = t3140 * t9656;
    let t121035 = t121034 * t1385;
    let t121043 = t32276 * t1404 * t32278;
    (t121019, t121024, t121029, t121034, t121035, t121043)
}
