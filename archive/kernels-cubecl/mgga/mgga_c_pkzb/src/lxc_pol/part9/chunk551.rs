//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 551/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk551<F: Float>(t2258: F, t2281: F, t2172: F, t2175: F, t2187: F) -> (F, F, F) {
    let t2282 = t2258 * t2281;
    let t2285 = F::cast_from(0.12361111111111111111e-1_f64) * t2172;
    let t2288 = t2285 - F::cast_from(0.18541666666666666667e-1_f64) * t2175 + F::cast_from(0.278125e-1_f64) * t2187;
    (t2282, t2285, t2288)
}
