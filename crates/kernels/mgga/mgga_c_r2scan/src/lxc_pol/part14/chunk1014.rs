//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1014/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1014<F: Float>(t12123: F, t12133: F, t12146: F, t12159: F, t12168: F, t12180: F, t12188: F, t12194: F, t797: F, t1048: F, t499: F, t11002: F, t1115: F, t983: F) -> (F, F, F, F, F) {
    let t12197 = t12123 + t12133 + t12146 + t12159 + t12168 + t12180 + t12188 + t12194;
    let t12198 = t12197 * t797;
    let t12200 = t1048 * t499 * t12198;
    let t12201 = t12200 / F::new(4.0);
    let t12203 = t11002 * t1115 * t983;
    (t12197, t12198, t12200, t12201, t12203)
}
