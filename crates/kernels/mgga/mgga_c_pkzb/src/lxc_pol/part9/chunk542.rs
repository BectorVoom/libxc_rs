//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 542/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk542<F: Float>(t2198: F, t2242: F, t2240: F, t2172: F, t2175: F, t2187: F, t858: F, t862: F) -> (F, F, F, F, F) {
    let t2243 = t2198 * t2242;
    let t2245 = F::cast_from(0.16081979498692535067e2_f64) * t2240 * t2243;
    let t2246 = F::cast_from(0.22831111111111111111e-1_f64) * t2172;
    let t2249 = t2246 - F::cast_from(0.34246666666666666666e-1_f64) * t2175 + F::cast_from(0.5137e-1_f64) * t2187;
    let t2252 = t858 * t862;
    (t2243, t2245, t2246, t2249, t2252)
}
