//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 920/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk920<F: Float>(t1231: F, t1861: F, t26: F, t1819: F, t2998: F, t555: F, t1808: F, t2997: F, t1181: F, t1874: F, t1877: F, t1804: F, t1807: F, t19: F, t558: F, t6201: F, t6204: F, t6207: F, t6216: F, t8183: F, t8187: F, t8189: F, t8193: F, t8199: F, t8201: F) -> (F, F, F, F, F, F, F) {
    let t8204 = t1231 * t1861;
    let t8205 = t26 * t8204;
    let t8210 = t555 * t1819 * t2998 / F::new(96.0);
    let t8211 = t2997 * t1808;
    let t8216 = t1181 * t1874 / F::new(32.0);
    let t8218 = t1181 * t1877 / F::new(32.0);
    let t8219 = -t6201 / F::new(96.0) - t6204 / F::new(96.0) - t6207 / F::new(192.0) - t6216 / F::new(144.0) - t8183 + F::new(7.0) / F::new(96.0) * t8187 - t555 * t558 * t8189 / F::new(64.0) - t555 * t558 * t8193 / F::new(32.0) - t8199 - F::new(3.0) / F::new(32.0) * t19 * t8201 - F::new(3.0) / F::new(64.0) * t19 * t8205 - t8210 - t1804 * t1807 * t8211 / F::new(48.0) - t8216 - t8218;
    (t8204, t8205, t8210, t8211, t8216, t8218, t8219)
}
