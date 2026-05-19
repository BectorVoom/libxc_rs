//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 132/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk132<F: Float>(t345: F, t348: F, t351: F, t355: F) -> (F, F, F) {
    let t370 = F::new(0.705945e1) * t348 + F::new(0.1549425e1) * t345 + F::new(0.420775e0) * t351 + F::new(0.1562925e0) * t355;
    let t373 = F::new(1.0) + F::cast_from(0.32163958997385070134e2_f64) / t370;
    let t374 = F::ln(t373);
    (t370, t373, t374)
}
