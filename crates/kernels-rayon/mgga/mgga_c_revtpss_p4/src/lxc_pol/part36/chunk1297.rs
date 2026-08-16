//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1297/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1297(t112757: f64, t7642: f64, t5219: f64, t8190: f64, t30882: f64, t7635: f64, t30923: f64, t3801: f64, t2172: f64, t6936: f64, t1921: f64, t8240: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t112880 = t7642 * t112757;
    let t112902 = t5219 * t8190;
    let t112943 = t30882 * t7635;
    let t112958 = t30923 * t3801;
    let t113019 = t6936 * t2172;
    let t113022 = t8240 * t1921;
    (t112880, t112902, t112943, t112958, t113019, t113022)
}
