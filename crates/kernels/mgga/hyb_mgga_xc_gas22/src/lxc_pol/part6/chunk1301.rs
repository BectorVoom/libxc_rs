//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1301/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1301<F: Float>(t10327: F, t1890: F, t4010: F, t6012: F, t4006: F, t10315: F, t1238: F, t2028: F, t20346: F, t2052: F, t2055: F, t2060: F, t24216: F, t24218: F, t24220: F, t27374: F, t3023: F, t3171: F, t3177: F, t3938: F, t457: F, t572: F, t6299: F, t699: F) -> F {
    let t28291 = t1890 * t10327;
    let t28297 = t6012 * t4010;
    let t28299 = t6012 * t4006;
    let t28309 = -F::new(4.0) / F::new(81.0) * t24216 + F::new(2.0) / F::new(27.0) * t24218 + F::new(2.0) / F::new(27.0) * t572 * t3171 * t6299 * t3938 * t2028 - t572 * t3177 * t10315 * t2028 / F::new(9.0) + F::new(4.0) / F::new(81.0) * t3023 * t2052 * t2055 * t1238 + t20346 + F::new(142.0) / F::new(243.0) * t24220 + t28291 / F::new(81.0) - F::new(4.0) / F::new(27.0) * t3023 * t699 * t2060 * t1238 - F::new(2.0) / F::new(243.0) * t28297 + F::new(4.0) / F::new(243.0) * t28299 + F::new(8.0) / F::new(27.0) * t27374 * t699 * t2060 * t457 - F::new(8.0) / F::new(81.0) * t27374 * t2052 * t2055 * t457;
    t28309
}
