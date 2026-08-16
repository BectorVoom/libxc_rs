//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1953/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1953(t1398: f64, t543: f64, t6918: f64, t1955: f64, t27883: f64, t1444: f64, t6844: f64, t1903: f64, t5658: f64, t1032: f64, t6888: f64, t1426: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108206 = t6918 * t1398 * t543;
    let t108225 = t1955 * t27883;
    let t108244 = t6844 * t1444;
    let t108259 = t1903 * t5658 * t543;
    let t108277 = t6888 * t1032;
    let t108278 = t108277 * t1426;
    (t108206, t108225, t108244, t108259, t108277, t108278)
}
