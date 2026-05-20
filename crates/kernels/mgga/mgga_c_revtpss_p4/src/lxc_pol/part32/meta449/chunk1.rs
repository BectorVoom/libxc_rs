//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1625/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1625<F: Float>(t13789: F, t22298: F, t14038: F, t14040: F, t14042: F, t14043: F, t14049: F, t14053: F, t14057: F, t1410: F, t22285: F, t22289: F, t22292: F, t22295: F, t3934: F, t9977: F) -> (F, F) {
    let t22299 = t13789 * t22298;
    let t22304 = -F::cast_from(0.20007875121765877254e-2_f64) * t22285 + F::cast_from(0.42874018118069736972e-2_f64) * t1410 * t22289 + F::cast_from(0.10003937560882938627e-2_f64) * t22292 - F::cast_from(0.85748036236139473945e-2_f64) * t3934 * t22295 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t22299 - t14038 - t14040 + t14042 + F::cast_from(0.27104001498285508386e-3_f64) * t14043 - t14049 + t14053 - t14057 + F::cast_from(0.13552000749142754193e-3_f64) * t9977;
    (t22299, t22304)
}
