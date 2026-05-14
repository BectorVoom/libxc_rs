//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 997/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk997<F: Float>(t72943: F, t72994: F, t39: F, t5585: F, t37481: F, t1611: F, t8: F, t668: F, t771: F, t1402: F, t1771: F, t6005: F, t24228: F, t24237: F, t6061: F, t24257: F, t5999: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t72995 = t72943 + t72994;
    let t92354 = t39 * t5585;
    let t93053 = t5585 * t37481;
    let t93076 = t8 * t1611;
    let t96339 = t771 * t668;
    let t96360 = t1402 * t1771;
    let t96361 = t96360 * t6005;
    let t96363 = t24237 * t24228;
    let t96382 = t6061 * t771;
    let t96392 = t24257 * t5999;
    (t72995, t92354, t93053, t93076, t96339, t96360, t96361, t96363, t96382, t96392)
}
