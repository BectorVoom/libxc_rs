//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1114/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1114(t18957: f64, t2396: f64, t3206: f64, t6471: f64, t926: f64, t6372: f64, t6475: f64, t2370: f64, t6506: f64, t2368: f64, t5728: f64, t154: f64, t2347: f64, t385: f64, t6106: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18958 = t2396 * t18957;
    let t18963 = t3206 * t926 * t6471;
    let t18967 = t3206 * t6475 * t6372;
    let t18974 = t2370 * t6506;
    let t18979 = t2368 * t5728;
    let t18987 = t385 * t154 * t2347 * t6106;
    (t18958, t18963, t18967, t18974, t18979, t18987)
}
