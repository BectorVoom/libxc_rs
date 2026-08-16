//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 526/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk526(t940: f64, t238: f64, t242: f64, t2466: f64, t343: f64, t2457: f64, t2468: f64, t2486: f64, t2491: f64, t2493: f64, t2497: f64, t2499: f64, t2503: f64, t2505: f64) -> (f64, f64, f64, f64, f64) {
    let t2507 = t940 * t940;
    let t2509 = t238 * t242 * t2507;
    let t2511 = t343 * t2466;
    let t2513 = t238 * t242 * t2511;
    let t2515 = -0.9494625e0_f64 * t2486 + 0.1898925e1_f64 * t2491 + t2493 - 0.59793333333333333334e0_f64 * t2457 + 0.8969e0_f64 * t2468 + 0.15358125e0_f64 * t2497 + 0.3071625e0_f64 * t2499 + t2503 - 0.32862666666666666666e0_f64 * t2505 + 0.24647e0_f64 * t2509 + 0.24647e0_f64 * t2513;
    (t2507, t2509, t2511, t2513, t2515)
}
