//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 66/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk66<F: Float>(t183: F, t190: F, t149: F, t65: F, t66: F, t67: F) -> (F, F, F, F, F, F) {
    let t192 = 1.0 * t183 * t190;
    let t194 = 0.3529725e1 * t149 + t65 + t66 + t67;
    let t197 = 1.0 + 0.32163958997385070134e2 / t194;
    let t198 = f64::ln(t197);
    let t200 = t194 * t194;
    let t201 = 1.0 / t200;
    (t192, t194, t197, t198, t200, t201)
}
