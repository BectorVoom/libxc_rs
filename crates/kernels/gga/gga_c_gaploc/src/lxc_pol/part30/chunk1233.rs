//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1233/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1233<F: Float>(t10396: F, t20565: F, t31586: F, t4820: F, t6824: F, t31591: F, t10399: F, t21272: F, t2478: F, t2792: F, t6576: F, t7047: F, t993: F, t1540: F, t196: F, t20157: F, t3176: F, t4525: F, t8124: F) -> (F, F, F, F, F, F, F) {
    let t34706 = 0.15889106645266856297e0 * t20565 * t10396;
    let t34709 = 0.15889106645266856297e0 * t6824 * t4820 * t31586;
    let t34712 = 0.15889106645266856297e0 * t6824 * t4820 * t31591;
    let t34713 = t21272 * t10399;
    let t34714 = 0.38342925953920749676e0 * t34713;
    let t34716 = t6576 * t2792 * t2478;
    let t34717 = 0.38342925953920749676e0 * t34716;
    let t34719 = t6576 * t993 * t7047;
    let t34720 = 0.19171462976960374838e0 * t34719;
    let t34726 = 0.12269736305254639897e2 * t196 * t4525 * t20157 * t8124 * t3176 * t1540;
    (t34706, t34709, t34712, t34714, t34717, t34720, t34726)
}
