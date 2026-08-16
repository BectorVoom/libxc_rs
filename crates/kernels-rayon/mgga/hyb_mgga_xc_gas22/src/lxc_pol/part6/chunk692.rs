//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 692/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk692(t1392: f64, t940: f64, t238: f64, t242: f64, t343: f64, t3470: f64, t2457: f64, t2493: f64, t2503: f64, t2505: f64, t3461: f64, t3472: f64, t3486: f64, t3491: f64, t3497: f64, t3499: f64, t3503: f64) -> (f64, f64, f64, f64, f64) {
    let t3505 = t940 * t1392;
    let t3507 = t238 * t242 * t3505;
    let t3509 = t343 * t3470;
    let t3511 = t238 * t242 * t3509;
    let t3513 = -0.9494625e0_f64 * t3486 + 0.1898925e1_f64 * t3491 + t2493 - 0.29896666666666666667e0_f64 * t2457 - 0.29896666666666666667e0_f64 * t3461 + 0.8969e0_f64 * t3472 + 0.15358125e0_f64 * t3497 + 0.3071625e0_f64 * t3499 + t2503 - 0.16431333333333333333e0_f64 * t2505 - 0.16431333333333333333e0_f64 * t3503 + 0.24647e0_f64 * t3507 + 0.24647e0_f64 * t3511;
    (t3505, t3507, t3509, t3511, t3513)
}
