//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 761/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk761<F: Float>(t2057: F, t739: F, t2045: F, t735: F, t291: F, t3: F, t197: F, t290: F, t297: F, t2082: F, t775: F, t2065: F, t771: F, t1485: F, t178: F, t301: F) -> (F, F, F, F, F, F, F) {
    let t5595 = t2057 * t739;
    let t5597 = t735 * t2045;
    let t5599 = t291 * t291;
    let t5601 = 1.0 / t3 / t5599;
    let t5604 = t290 * t197 * t5601 * t297;
    let t5607 = t2082 * t775;
    let t5609 = t771 * t2065;
    let t5612 = t178 * t1485 * t301;
    (t5595, t5597, t5601, t5604, t5607, t5609, t5612)
}
