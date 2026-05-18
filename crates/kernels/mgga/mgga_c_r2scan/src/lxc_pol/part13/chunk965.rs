//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 965/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk965<F: Float>(t1563: F, t797: F, t10997: F, t3275: F, t113: F, t1561: F) -> (F, F, F) {
    let t10998 = t797 * t1563;
    let t11000 = t3275 * t10997 * t10998;
    let t11001 = F::new(45.0) / F::new(64.0) * t11000;
    let t11002 = t113 * t1561;
    (t10998, t11001, t11002)
}
