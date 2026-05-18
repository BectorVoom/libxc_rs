//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1242/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1242<F: Float>(t40185: F, t41734: F, t41735: F, t41736: F, t43602: F, t43606: F, t43609: F, t43612: F, t43616: F, t43619: F, t43622: F, t43625: F) -> F {
    let t44471 = t41734 + F::new(0.46230515946956099003e0) * t43602 + t41735 + t41736 - F::new(0.62295486109113302474e-1) * t40185 + F::new(0.43663693315433241794e-2) * t43606 - F::new(0.93149212406257582492e-1) * t43609 - F::new(0.52396431978519890152e-1) * t43612 + F::new(0.55889527443754549496e0) * t43616 + F::new(0.2600466522016280569e0) * t43619 - F::new(0.34672886960217074252e0) * t43622 - F::new(0.10401866088065122276e1) * t43625;
    t44471
}
