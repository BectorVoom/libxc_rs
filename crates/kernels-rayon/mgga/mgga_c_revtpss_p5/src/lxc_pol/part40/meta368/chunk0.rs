//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1297/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1297(t15525: f64, t4733: f64, t981: f64, t15495: f64, t300: f64, t15234: f64, t964: f64, t973: f64, t2986: f64, t4707: f64, t974: f64, t11506: f64, t1633: f64) -> (f64, f64, f64, f64, f64) {
    let t15526 = t15525 * t4733;
    let t15528 = 0.34631718211362927518e2_f64 * t981 * t15526;
    let t15530 = 0.19751673498613801407e-1_f64 * t300 * t15495;
    let t15534 = t964 * t15234 * t973;
    let t15536 = 0.5848223622634646207e0_f64 * t981 * t15534;
    let t15537 = t2986 * t4707;
    let t15538 = t15537 * t974;
    let t15540 = 0.23392894490538584828e1_f64 * t981 * t15538;
    let t15541 = t11506 * t1633;
    (t15528, t15530, t15536, t15540, t15541)
}
