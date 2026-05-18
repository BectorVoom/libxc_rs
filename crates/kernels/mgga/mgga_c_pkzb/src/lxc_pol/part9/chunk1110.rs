//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1110/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1110<F: Float>(t300: F, t6404: F, t394: F, t6406: F, t2099: F, t6459: F, t6463: F, t2320: F, t6224: F, t2255: F, t2277: F, t356: F) -> (F, F, F, F, F) {
    let t18661 = t300 * t6404;
    let t18662 = t394 * t6406;
    let t18668 = t6459 * t2099 * t6463;
    let t18679 = t6224 * t2320;
    let t18706 = t356 / t2277 / t2255;
    (t18661, t18662, t18668, t18679, t18706)
}
