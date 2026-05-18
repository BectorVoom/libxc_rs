//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 971/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk971<F: Float>(t11056: F, t819: F, t11032: F, t11035: F, t11037: F, t11039: F, t11041: F, t11043: F, t11046: F, t11048: F, t11052: F, t11054: F) -> (F, F) {
    let t11057 = t819 * t11056;
    let t11058 = F::new(11.0) / F::new(9.0) * t11057;
    let t11059 = -t11032 - t11035 - t11037 / F::new(4.0) + t11039 / F::new(8.0) - t11041 / F::new(8.0) + t11043 / F::new(2.0) + t11046 - F::new(3.0) / F::new(4.0) * t11048 - t11052 + t11054 / F::new(4.0) - t11058;
    (t11058, t11059)
}
