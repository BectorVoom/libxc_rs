//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1083/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1083<F: Float>(t10534: F, t10549: F, t6530: F, t6552: F, t8676: F, t8751: F, t284: F, t4108: F, t787: F, t811: F, t1347: F, t8865: F) -> (F, F, F, F, F) {
    let t10551 = -t6552 + F::cast_from(0.12361111111111111111e-1_f64) * t6530 + F::cast_from(0.24722222222222222223e-1_f64) * t8676 - t8751 - F::cast_from(0.92708333333333333333e-2_f64) * t10534 + F::new(0.278125e-1) * t10549;
    let t10552 = t10551 * t284;
    let t10555 = t4108 * t787;
    let t10557 = F::new(1.0) * t10555 * t811;
    let t10559 = F::new(2.0) * t8865 * t1347;
    (t10551, t10552, t10555, t10557, t10559)
}
