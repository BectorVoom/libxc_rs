//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2428/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2428<F: Float>(t3566: F, t5462: F, t5477: F, t576: F, t588: F, t15: F, t27: F, t11: F, t22: F, t10276: F, t584: F, t596: F) -> (F, F, F, F, F, F, F) {
    let t45859 = t3566 * t5462;
    let t45863 = t3566 * t5477;
    let t45928 = t576 * t588;
    let t45931 = F::new(120.0) * t15 * t27;
    let t45933 = F::new(24.0) * t11 * t22;
    let t45934 = t10276 * t588;
    let t45938 = t584 * t596;
    (t45859, t45863, t45928, t45931, t45933, t45934, t45938)
}
