//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1336/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1336<F: Float>(t1617: F, t3822: F, t4915: F, t12052: F, t23726: F, t2011: F, t3808: F, t30472: F, t3483: F, t12038: F, t575: F, t687: F) -> (F, F, F, F, F) {
    let t36098 = F::new(6.0) * t4915 * t3822 * t1617;
    let t36100 = F::new(12.0) * t23726 * t12052;
    let t36103 = F::new(6.0) * t4915 * t3808 * t2011;
    let t36105 = F::new(4.0) * t30472 * t3483;
    let t36106 = t12038 * t575;
    let t36108 = F::new(2.0) * t36106 * t687;
    (t36098, t36100, t36103, t36105, t36108)
}
