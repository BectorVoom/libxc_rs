//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 889/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk889<F: Float>(t10614: F, t10618: F, t10621: F, t10625: F, t10629: F, t10633: F, t10637: F, t10643: F, t10653: F, t10657: F, t10925: F, t10975: F, t11026: F, t354: F, t1266: F, t321: F) -> (F, F, F) {
    let t11028 = -t10614 + 0.15243824895787514157e-3 * t10643 + t10618 - t10621 + t10625 - t10629 - t10633 + 0.72042316457491791906e-3 * t10653 + t10637 - t10657 + t10925 + t10975 + t11026;
    let t11029 = t354 * t11028;
    let t11031 = t1266 * t321;
    (t11028, t11029, t11031)
}
