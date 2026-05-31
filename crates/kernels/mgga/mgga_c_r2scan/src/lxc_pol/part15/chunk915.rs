//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 915/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk915<F: Float>(t1543: F, t797: F, t10610: F, t3263: F, t1561: F, t3347: F) -> (F, F, F) {
    let t10611 = t797 * t1543;
    let t10613 = t10610 * t3263 * t10611;
    let t10614 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t10613;
    let t10615 = t1561 * t3347;
    (t10611, t10614, t10615)
}
