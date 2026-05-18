//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 134/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk134<F: Float>(t345: F, t348: F, t351: F, t355: F) -> (F, F, F) {
    let t383 = F::new(0.51785e1) * t348 + F::new(0.905775e0) * t345 + F::new(0.1100325e0) * t351 + F::new(0.1241775e0) * t355;
    let t386 = F::new(1.0) + F::new(0.29608749977793437516e2) / t383;
    let t387 = f64::ln(t386);
    (t383, t386, t387)
}
