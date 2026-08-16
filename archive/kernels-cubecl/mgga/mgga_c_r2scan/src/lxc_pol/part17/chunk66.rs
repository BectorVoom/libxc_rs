//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 66/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk66<F: Float>(t183: F, t190: F, t149: F, t65: F, t66: F, t67: F) -> (F, F, F, F, F, F) {
    let t192 = F::cast_from(1.0_f64) * t183 * t190;
    let t194 = F::cast_from(0.3529725e1_f64) * t149 + t65 + t66 + t67;
    let t197 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t194;
    let t198 = F::ln(t197);
    let t200 = t194 * t194;
    let t201 = F::cast_from(1.0_f64) / t200;
    (t192, t194, t197, t198, t200, t201)
}
