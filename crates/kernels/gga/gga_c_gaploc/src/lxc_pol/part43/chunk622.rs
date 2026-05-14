//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 622/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk622<F: Float>(t12834: F, t2268: F, t2765: F, t3137: F, t3145: F, t8045: F, t2798: F, t3207: F, t1016: F, t9243: F, t3366: F, t6556: F, t4349: F, t1382: F, t12766: F, t1445: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12836 = 0.28455006635676149599e-1 * t2268 * t12834;
    let t12840 = t2765 * t3137;
    let t12842 = 0.85365019907028448797e-1 * t2268 * t12840;
    let t12849 = 2.0 * t8045 * t3145;
    let t12850 = t2798 * t3207;
    let t12851 = t9243 * t1016;
    let t12853 = 4.0 * t6556 * t3366;
    let t12856 = t1016 * t3145;
    let t12858 = 6.0 * t4349 * t12856;
    let t12862 = t1016 * t3207;
    let t12864 = 2.0 * t1382 * t12862;
    let t12868 = t1445 * t12766;
    (t12836, t12840, t12842, t12849, t12850, t12851, t12853, t12856, t12858, t12862, t12864, t12868)
}
