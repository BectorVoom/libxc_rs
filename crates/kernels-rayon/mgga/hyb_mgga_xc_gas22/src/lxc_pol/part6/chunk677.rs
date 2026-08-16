//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 677/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk677(t3389: f64, t828: f64, t2167: f64, t2279: f64, t3300: f64, t3311: f64) -> (f64, f64) {
    let t3390 = t3389 * t828;
    let t3396 = t2279 - 0.92708333333333333333e-2_f64 * t2167 - 0.92708333333333333333e-2_f64 * t3300 + 0.278125e-1_f64 * t3311;
    (t3390, t3396)
}
