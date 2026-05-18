//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1114/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1114<F: Float>(t10926: F, t10952: F, t968: F, t949: F, t4273: F, t7070: F, t10898: F, t10913: F, t6967: F, t6969: F, t9008: F, t9012: F) -> (F, F, F, F, F) {
    let t10953 = t10926 + t10952;
    let t10954 = t10953 * t968;
    let t10956 = F::new(1.0) * t949 * t10954;
    let t10958 = F::new(0.16081979498692535067e2) * t7070 * t4273;
    let t10963 = -t6967 + F::new(0.12361111111111111111e-1) * t6969 + F::new(0.24722222222222222223e-1) * t9008 - t9012 - F::new(0.92708333333333333333e-2) * t10898 + F::new(0.278125e-1) * t10913;
    (t10953, t10954, t10956, t10958, t10963)
}
