//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 689/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk689(t222: f64, t3470: f64, t37: f64, t2455: f64, t2457: f64, t3461: f64, t361: f64, t1396: f64, t948: f64) -> (f64, f64, f64, f64) {
    let t3472 = t222 * t37 * t3470;
    let t3474 = t2455 - 0.17808333333333333333e-1_f64 * t2457 - 0.17808333333333333333e-1_f64 * t3461 + 0.53425e-1_f64 * t3472;
    let t3476 = 0.621814e-1_f64 * t3474 * t361;
    let t3477 = t1396 * t948;
    (t3472, t3474, t3476, t3477)
}
