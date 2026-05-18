//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 872/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk872<F: Float>(t5560: F, t5790: F, t7332: F, t7434: F, t7435: F, t9178: F, t9180: F, t9185: F, t9189: F, t9192: F, t9196: F, t9200: F) -> F {
    let t9388 = F::new(0.15358125e0) * t9178 + F::new(0.3071625e0) * t9180 - t5790 + F::new(0.27385555555555555556e0) * t5560 + F::new(0.5477111111111111111e0) * t7332 - t7434 - t7435 - F::new(0.16431333333333333333e0) * t9185 + F::new(0.49294e0) * t9189 - F::new(0.16431333333333333333e0) * t9192 + F::new(0.24647e0) * t9196 + F::new(0.24647e0) * t9200;
    t9388
}
