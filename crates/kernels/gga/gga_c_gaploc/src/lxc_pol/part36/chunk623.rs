//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 623/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk623<F: Float>(t10283: F, t921: F, t3145: F, t8045: F, t2798: F, t3207: F, t1016: F, t9243: F, t3366: F, t6556: F, t2355: F, t3418: F, t4349: F, t1382: F, t12762: F, t1445: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12846 = t10283 * t921;
    let t12847 = 2.0 * t12846;
    let t12849 = 2.0 * t8045 * t3145;
    let t12850 = t2798 * t3207;
    let t12851 = t9243 * t1016;
    let t12853 = 4.0 * t6556 * t3366;
    let t12854 = t2355 * t3418;
    let t12855 = 2.0 * t12854;
    let t12856 = t1016 * t3145;
    let t12858 = 6.0 * t4349 * t12856;
    let t12859 = t3418 * t921;
    let t12860 = t1382 * t12859;
    let t12861 = 4.0 * t12860;
    let t12862 = t1016 * t3207;
    let t12864 = 2.0 * t1382 * t12862;
    let t12865 = t1445 * t12762;
    (t12847, t12849, t12850, t12851, t12853, t12855, t12856, t12858, t12859, t12861, t12862, t12864, t12865)
}
