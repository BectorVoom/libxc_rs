//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 845/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk845(t828: f64, t9400: f64, t9942: f64, t595: f64, t66: f64, t240: f64, t247: f64, t550: f64, t548: f64, t4010: f64, t72: f64, t245: f64) -> (f64, f64, f64, f64, f64) {
    let t9944 = t9942 * t828 * t9400;
    let t9948 = 1.0_f64 / t66 / t595;
    let t9949 = t9948 * t240;
    let t9951 = t9949 * t550 * t247;
    let t9953 = 0.37792653007779990369e-1_f64 * t548 * t9951;
    let t9954 = t4010 * t72;
    let t9955 = t9954 * t245;
    (t9944, t9949, t9951, t9953, t9955)
}
