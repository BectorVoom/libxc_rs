//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 788/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk788<F: Float>(t276: F, t5589: F, t275: F, t4784: F, t2057: F, t739: F, t2045: F, t735: F, t291: F, t3: F, t197: F, t290: F, t297: F) -> (F, F, F, F, F, F) {
    let t5591 = F::new(5.0) / F::new(1296.0) * t276 * t5589;
    let t5592 = t4784 * t275;
    let t5595 = t2057 * t739;
    let t5597 = t735 * t2045;
    let t5599 = t291 * t291;
    let t5601 = F::new(1.0) / t3 / t5599;
    let t5604 = t290 * t197 * t5601 * t297;
    (t5591, t5592, t5595, t5597, t5601, t5604)
}
