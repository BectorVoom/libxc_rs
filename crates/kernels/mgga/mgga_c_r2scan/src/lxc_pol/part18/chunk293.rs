//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 293/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk293<F: Float>(t246: F, t381: F, t404: F, t408: F, t412: F, t459: F, t466: F, t470: F, t764: F, t765: F, t900: F, t902: F, t955: F, t970: F, t11: F, t5: F, t581: F, t966: F) -> (F,) {
    let t975 = t764 + 0.675260332e-1 * t765 * t970 - 0.285764e-1 * t246 * t955 - t381 - t404 + t408 + t412 - t900 - t459 - t902 + t466 + t470;
    let t978 = 5.0 * t5 * t11 * t966 - 45.0 * param_eta * t975 - t581;
    (t978,)
}
