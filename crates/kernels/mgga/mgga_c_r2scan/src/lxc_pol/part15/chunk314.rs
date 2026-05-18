//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 314/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk314<F: Float>(t1058: F, t1060: F, t783: F, t1051: F, t1056: F) -> F {
    let t1062 = t783 * t1058 * t1060;
    let t1064 = F::new(0.27439371595564631661e-1) * t1051 + F::new(0.43341108700271342816e-1) * t1056 - F::new(0.21831846657716620896e-2) * t1062;
    t1064
}
