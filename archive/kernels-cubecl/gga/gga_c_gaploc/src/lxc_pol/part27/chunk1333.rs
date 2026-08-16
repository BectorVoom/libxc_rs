//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1333/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1333<F: Float>(t34716: F, t6576: F, t7047: F, t993: F, t1540: F, t196: F, t20157: F, t3176: F, t4525: F, t8124: F, t1560: F, t31775: F) -> (F, F, F, F) {
    let t34717 = F::cast_from(0.38342925953920749676e0_f64) * t34716;
    let t34719 = t6576 * t993 * t7047;
    let t34720 = F::cast_from(0.19171462976960374838e0_f64) * t34719;
    let t34726 = F::cast_from(0.12269736305254639897e2_f64) * t196 * t4525 * t20157 * t8124 * t3176 * t1540;
    let t34730 = F::cast_from(0.27606906686822939768e2_f64) * t196 * t1560 * t20157 * t31775;
    (t34717, t34720, t34726, t34730)
}
