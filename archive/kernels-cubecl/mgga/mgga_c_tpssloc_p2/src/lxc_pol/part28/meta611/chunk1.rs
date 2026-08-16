//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1924/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1924<F: Float>(t26418: F, t6914: F, t7736: F, t80854: F, t81064: F, t22704: F, t22705: F, t26410: F, t26432: F, t6897: F, t794: F, t22642: F, t22690: F, t26395: F) -> (F, F, F, F, F) {
    let t90970 = t6914 * t26418;
    let t90980 = t81064 * t80854 * t7736;
    let t90983 = t22704 * t22705 * t26410;
    let t90987 = t6897 * t794 * t26432;
    let t90993 = t22642 * t22690 * t26395;
    (t90970, t90980, t90983, t90987, t90993)
}
