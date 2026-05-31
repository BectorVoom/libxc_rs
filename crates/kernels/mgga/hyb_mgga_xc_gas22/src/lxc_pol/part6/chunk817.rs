//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 817/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk817<F: Float>(t1145: F, t4576: F, t1117: F, t1134: F, t1144: F, t1149: F, t1167: F, t1169: F, t2903: F, t2922: F, t2927: F, t4541: F, t4545: F, t4550: F, t4553: F, t4556: F, t4559: F, t4562: F, t4565: F, t4568: F, t4571: F, t4574: F, t510: F, t513: F, t518: F, t538: F) -> (F, F) {
    let t4577 = t1145 * t4576;
    let t4582 = F::cast_from(2.0_f64) * t1117 * t4565 + F::cast_from(6.0_f64) * t1134 * t4559 + F::cast_from(3.0_f64) * t1144 * t4541 - F::cast_from(3.0_f64) * t1149 * t4545 + t1167 * t4541 - t1169 * t4545 + F::cast_from(30.0_f64) * t2903 * t4553 - F::cast_from(36.0_f64) * t2922 * t4577 - F::cast_from(4.0_f64) * t2927 * t4577 + F::cast_from(6.0_f64) * t510 * t4550 + F::cast_from(42.0_f64) * t518 * t4556 - F::cast_from(6.0_f64) * t518 * t4562 - F::cast_from(2.0_f64) * t510 * t4568 + F::cast_from(2.0_f64) * t4571 * t513 + t4574 * t538;
    (t4577, t4582)
}
