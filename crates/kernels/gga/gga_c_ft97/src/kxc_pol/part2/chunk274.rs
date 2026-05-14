//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 274/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk274<F: Float>(t1073: F, t637: F, t639: F, t1068: F, t629: F, t631: F, t184: F, t21: F, t669: F, t992: F, t666: F, t89: F, t668: F) -> (F, F, F, F, F, F, F) {
    let t1075 = t637 * t639 * t1073;
    let t1078 = t629 + t631 * t1068 / 6.0 + t631 * t1075 / 2.0;
    let t1079 = t1078 * t184;
    let t1080 = t1079 * t21;
    let t1087 = t669 * t992;
    let t1089 = t89 * t666 * t1087;
    let t1091 = t668 * t992;
    (t1075, t1078, t1079, t1080, t1087, t1089, t1091)
}
