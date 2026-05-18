//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1441/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1441<F: Float>(t1129: F, t11536: F, t11544: F, t26552: F, t26564: F, t26579: F, t31225: F, t31229: F, t31304: F, t31310: F, t31311: F, t31317: F, t31322: F, t31330: F, t9632: F, t9642: F, t9667: F, t9765: F, t9769: F, t9773: F) -> F {
    let t31337 = F::new(504.0) * t9773 * t31304 + F::new(24.0) * t9765 * t31304 + F::new(10000.0) / F::new(81.0) * t31310 * t31311 - F::new(360.0) * t9769 * t11544 * t1129 + F::new(504.0) * t9773 * t31317 + F::new(24.0) * t9765 * t31317 - F::new(96.0) * t26552 * t31322 - F::new(1440.0) * t26579 * t11536 * t1129 - F::new(4032.0) * t26564 * t31322 + F::new(1408.0) / F::new(81.0) * t9642 * t31330 - F::new(6400.0) / F::new(81.0) * t9632 * t31229 - F::new(1408.0) / F::new(243.0) * t9667 * t31225;
    t31337
}
