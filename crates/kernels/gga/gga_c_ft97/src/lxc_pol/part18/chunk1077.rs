//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1077/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1077<F: Float>(t1882: F, t23291: F, t23155: F, t5733: F, t8232: F, t5724: F, t23339: F, t47660: F, t23212: F, t23208: F, t23141: F, t23137: F, t1786: F, t5710: F, t23341: F, t8392: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t91682 = t1882 * t23291;
    let t91684 = t1882 * t23155;
    let t91705 = t8232 * t5733;
    let t91718 = t8232 * t5724;
    let t91739 = t47660 * t23339;
    let t91743 = t1882 * t23212;
    let t91745 = t1882 * t23208;
    let t91754 = t1882 * t23141;
    let t91760 = t1882 * t23137;
    let t91771 = t1786 * t5710;
    let t91783 = t8392 * t23341;
    (t91682, t91684, t91705, t91718, t91739, t91743, t91745, t91754, t91760, t91771, t91783)
}
