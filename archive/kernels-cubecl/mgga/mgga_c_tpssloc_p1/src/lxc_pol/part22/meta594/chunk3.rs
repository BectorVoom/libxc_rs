//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2114/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2114<F: Float>(t10277: F, t1043: F, t10216: F, t3061: F, t2770: F, t376: F, t1540: F, t9698: F) -> (F, F, F, F) {
    let t47775 = t1043 * t10277;
    let t47779 = t3061 * t10216;
    let t47783 = t376 * t2770;
    let t47787 = t9698 * t1540;
    (t47775, t47779, t47783, t47787)
}
