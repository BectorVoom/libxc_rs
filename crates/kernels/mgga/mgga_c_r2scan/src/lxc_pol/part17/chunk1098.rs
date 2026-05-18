//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1098/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1098<F: Float>(t10710: F, t25503: F, t37658: F, t11816: F, t37880: F, t10772: F, t10810: F, t2568: F, t10768: F, t8129: F, t2604: F, t625: F) -> (F, F, F, F, F) {
    let t39443 = t37658 * t10710 * t25503;
    let t39445 = t37880 * t11816;
    let t39458 = t10772 * t10810 * t2568;
    let t39464 = t10768 * t8129;
    let t39469 = t2604 * t625;
    (t39443, t39445, t39458, t39464, t39469)
}
