//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1057/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1057<F: Float>(t43: F, t10076: F, t1211: F, t1226: F, t3068: F, t3108: F, t3876: F, t3912: F, t616: F, t635: F, t72: F, t88: F, t9953: F, t9997: F, t9999: F) -> F {
    let t44 = F::new(0.135e1) <= t43;
    let t10080 = piecewise3::<F>(t44, t9953 + t9997, -F::new(8.0) / F::new(3.0) * t9999 * t88 - F::new(8.0) / F::new(3.0) * t3876 * t635 - F::new(16.0) / F::new(3.0) * t3068 * t1226 - F::new(16.0) / F::new(3.0) * t1211 * t3108 - F::new(8.0) / F::new(3.0) * t616 * t3912 - F::new(8.0) / F::new(3.0) * t72 * t10076);
    t10080
}
