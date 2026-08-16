//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1340/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1340<F: Float>(t23186: F, t82031: F, t1902: F, t2678: F, t22723: F, t23163: F, t23165: F, t10046: F, t1880: F, t1894: F, t214: F, t1879: F, t80845: F) -> (F, F, F, F, F, F) {
    let t82032 = t82031 * t23186;
    let t82034 = t1902 * t2678;
    let t82038 = t22723 * t23163;
    let t82039 = t82038 * t23165;
    let t82043 = t1880 * t214 * t1894 * t10046;
    let t82045 = t80845 * t1879;
    (t82032, t82034, t82038, t82039, t82043, t82045)
}
