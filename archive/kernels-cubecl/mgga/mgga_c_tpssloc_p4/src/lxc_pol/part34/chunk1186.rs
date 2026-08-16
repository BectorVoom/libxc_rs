//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1186/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1186<F: Float>(t5398: F, t72: F, t79: F, t20218: F, t605: F, t1410: F, t19299: F, t1799: F, t6463: F, t20347: F, t88: F, t20304: F, t81446: F) -> (F, F, F, F, F, F) {
    let t106853 = t72 * t79 * t5398;
    let t106855 = t605 * t20218;
    let t106862 = t19299 * t1410;
    let t106902 = t1799 * t6463;
    let t106935 = t88 * t20347;
    let t106944 = t81446 * t20304;
    (t106853, t106855, t106862, t106902, t106935, t106944)
}
