//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 406/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk406<F: Float>(t1578: F, t1580: F, t2318: F, t2322: F, t2328: F, t535: F, t541: F) -> (F,) {
    let t2331 = 0.2698618307426597582e-1 * t2318 * t541 + t1578 + 0.89953943580886586067e-2 * t1580 * t2322 - 0.2698618307426597582e-1 * t535 * t2328;
    (t2331,)
}
