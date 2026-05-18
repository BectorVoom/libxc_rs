//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 235/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk235<F: Float>(t1001: F, t1003: F, t286: F, t285: F, t293: F, t984: F, t989: F, t991: F, t996: F) -> (F, F) {
    let t1004 = t1001 * t1003;
    let t1005 = t286 * t1004;
    let t1008 = -t984 * t293 / F::new(36.0) + t989 + t991 * t996 / F::new(288.0) - t285 * t1005 / F::new(96.0);
    (t1004, t1008)
}
