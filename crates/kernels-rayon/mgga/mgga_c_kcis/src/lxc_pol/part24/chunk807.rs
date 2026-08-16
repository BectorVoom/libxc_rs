//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 807/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk807(t14874: f64, t1780: f64, t245: f64, t3393: f64, t5155: f64, t330: f64, t4920: f64, t5139: f64, t5147: f64, t8931: f64, t2943: f64, t365: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14875 = t14874 * sigma0;
    let t14907 = t1780 * t245;
    let t14913 = t3393 * t5155;
    let t14915 = t4920 * t330;
    let t14926 = 0.35374814814814814814e-1_f64 * t3393 * t5139;
    let t14927 = t8931 * t5147;
    let t14940 = t365 * t2943;
    (t14875, t14907, t14913, t14915, t14926, t14927, t14940)
}
