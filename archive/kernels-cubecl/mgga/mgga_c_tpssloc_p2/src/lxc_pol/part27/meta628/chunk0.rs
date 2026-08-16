//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2113/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2113<F: Float>(t25373: F, t86713: F, t25: F, t40772: F, t1530: F, t2749: F, t1408: F, t2752: F, t13487: F, t22960: F, t58071: F, t2: F) -> (F, F, F, F, F, F) {
    let t86714 = t25373 * t86713;
    let t86716 = t40772 * t25;
    let t86717 = t1530 * t2749;
    let t86718 = t86716 * t86717;
    let t86721 = t2752 * t1408;
    let t86722 = t86721 * t13487;
    let t86727 = t22960 * t58071;
    let t86730 = t2752 * t2;
    (t86714, t86717, t86718, t86722, t86727, t86730)
}
