//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 916/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk916<F: Float>(t43: F, t1211: F, t1226: F, t1947: F, t1990: F, t3068: F, t3108: F, t616: F, t635: F, t72: F, t8021: F, t8059: F, t8061: F, t8141: F, t88: F) -> F {
    let t44 = F::cast_from(0.135e1_f64) <= t43;
    let t8145 = piecewise3::<F>(t44, t8021 + t8059, -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t8061 * t88 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t3068 * t635 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1211 * t1990 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1947 * t1226 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t616 * t3108 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t72 * t8141);
    t8145
}
