//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1905/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1905<F: Float>(t22892: F, t7691: F, t90544: F, t1835: F, t254: F, t28200: F, t6883: F, t6888: F, t90739: F, t1845: F, t5187: F, t191: F, t192: F, t19537: F) -> (F, F, F, F, F, F) {
    let t97732 = t22892 * t90544 * t7691;
    let t97740 = t1835 * t254;
    let t97750 = t6883 * t28200;
    let t97766 = t6888 * t90739 * t7691;
    let t97789 = t5187 * t1845;
    let t97804 = t19537 * t191 * t192;
    (t97732, t97740, t97750, t97766, t97789, t97804)
}
