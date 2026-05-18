//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 486/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk486<F: Float>(t2925: F, t835: F, t2936: F, t769: F, t2089: F, t1022: F, t723: F) -> (F, F, F, F) {
    let t8469 = t835 * t2925;
    let t8478 = t769 * t2936;
    let t8483 = t2089 * t2925;
    let t8502 = t1022 * t723;
    (t8469, t8478, t8483, t8502)
}
