//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1153/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1153<F: Float>(t6689: F, t6691: F, t1922: F, t986: F, t1049: F, t225: F, t387: F) -> (F, F, F, F) {
    let t6692 = t6689 * t6691;
    let t6695 = t986 * t1922;
    let t6698 = t1049 * t225;
    let t6699 = t6698 * t387;
    (t6692, t6695, t6698, t6699)
}
