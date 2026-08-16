//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2189/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2189<F: Float>(t1808: F, t254: F, t1377: F, t6347: F, t1385: F, t22633: F, t22635: F, t1842: F, t90516: F, t1992: F, t26355: F, t90566: F) -> (F, F, F, F) {
    let t97626 = t1808 * t254;
    let t97637 = t1377 * t6347;
    let t97640 = t22633 * t22635 * t97637 * t1385;
    let t97644 = t22633 * t22635 * t90516 * t1842;
    let t97647 = t1992 * t90566 * t26355;
    (t97626, t97640, t97644, t97647)
}
