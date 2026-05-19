//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1348/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1348<F: Float>(t2234: F, t3356: F, t8853: F, t10658: F, t2228: F, t6562: F, t20829: F, t20832: F, t2189: F, t4113: F, t10648: F, t6497: F) -> (F, F, F, F) {
    let t29411 = F::cast_from(0.32163958997385070134e2_f64) * t2234 * t3356 * t8853;
    let t29414 = F::cast_from(0.51726012919273400301e3_f64) * t6562 * t10658 * t2228;
    let t29418 = F::cast_from(0.24955700379505800916e5_f64) * t20829 * t4113 * t20832 * t2189;
    let t29420 = F::new(4.0) * t6497 * t10648;
    (t29411, t29414, t29418, t29420)
}
