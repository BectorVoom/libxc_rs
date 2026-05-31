//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 262/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk262<F: Float>(t774: F, t803: F, t781: F, t792: F, t797: F, t807: F) -> (F, F, F) {
    let t842 = F::cast_from(0.301925e0_f64) * t774;
    let t845 = F::cast_from(0.82785e-1_f64) * t803;
    let t847 = F::cast_from(0.258925e1_f64) * t792 - t842 + F::cast_from(0.905775e0_f64) * t781 + F::cast_from(0.16504875e0_f64) * t797 - t845 + F::cast_from(0.248355e0_f64) * t807;
    (t842, t845, t847)
}
