//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1006/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1006<F: Float>(t495: F, t9369: F, t1057: F, t3649: F, t1523: F, t2813: F, t462: F, t1052: F, t3647: F, t7276: F, t7310: F, t7312: F, t7314: F, t7484: F, t7487: F, t7490: F, t7493: F, t8949: F, t9314: F, t9338: F) -> (F, F, F, F, F, F, F) {
    let t9370 = t9369 * t495;
    let t9372 = t1057 * t3649;
    let t9374 = t1523 * t2813;
    let t9375 = t462 * t9374;
    let t9376 = t1052 * t3649;
    let t9379 = F::cast_from(8.0_f64) * t1052 * t3647;
    let t9381 = F::cast_from(8.0_f64) * t1057 * t3647;
    let t9383 = t462 * t9370 - F::cast_from(24.0_f64) * t7276 + t7310 + t7312 + F::cast_from(2.0_f64) * t7314 + t7484 - t7487 - t7490 + t7493 - t8949 - t9314 - t9338 - F::cast_from(8.0_f64) * t9372 + t9375 + F::cast_from(8.0_f64) * t9376 + t9379 - t9381;
    (t9370, t9372, t9374, t9375, t9379, t9381, t9383)
}
