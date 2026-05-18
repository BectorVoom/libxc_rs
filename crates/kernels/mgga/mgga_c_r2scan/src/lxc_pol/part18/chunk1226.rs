//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1226/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1226<F: Float>(t39613: F, t40195: F, t8752: F, t39614: F, t8741: F, t1054: F, t5108: F, t9481: F, t6106: F, t8756: F, t29451: F, t37717: F) -> (F, F, F, F, F) {
    let t43631 = t39613 * t40195 * t8752;
    let t43635 = t39613 * t39614 * t8741;
    let t43638 = t5108 * t1054 * t9481;
    let t43641 = t6106 * t1054 * t8756;
    let t43643 = t37717 * t29451;
    (t43631, t43635, t43638, t43641, t43643)
}
