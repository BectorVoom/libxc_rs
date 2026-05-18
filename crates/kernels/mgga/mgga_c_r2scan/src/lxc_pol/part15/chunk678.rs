//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 678/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk678<F: Float>(t1561: F, t1563: F, t2259: F, t498: F, t1559: F, t282: F, t283: F) -> (F, F, F) {
    let t5078 = t1561 * t1563;
    let t5081 = t498 * t2259;
    let t5084 = t1559 * t282;
    let t5086 = F::new(1.0) / t283 / t5084;
    (t5078, t5081, t5086)
}
