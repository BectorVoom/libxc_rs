//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1746/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1746<F: Float>(t1177: F, t18225: F, t1193: F, t6109: F, t248: F, t3570: F, t6230: F, t3515: F, t1230: F, t18241: F, t11546: F, t18206: F) -> (F, F, F, F, F, F) {
    let t19087 = t1177 * t18225;
    let t19090 = t6109 * t1193;
    let t19095 = t248 * t3570 * t6230;
    let t19096 = t3515 * t19095;
    let t19101 = t248 * t1230 * t18241;
    let t19106 = t11546 * t18206;
    (t19087, t19090, t19095, t19096, t19101, t19106)
}
