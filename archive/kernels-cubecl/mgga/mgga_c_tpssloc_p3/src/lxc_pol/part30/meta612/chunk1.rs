//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2009/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2009<F: Float>(t135: F, t23631: F, t6688: F, t23617: F, t6680: F, t10889: F, t3033: F, t6753: F, t10510: F, t6755: F, t10870: F, t6765: F) -> (F, F, F, F, F) {
    let t82822 = t23631 * t135 * t6688;
    let t82830 = t6680 * t23617;
    let t82848 = t3033 * t6753 * t10889;
    let t82851 = t6755 * t10510;
    let t82875 = t6765 * t10870;
    (t82822, t82830, t82848, t82851, t82875)
}
