//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 722/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk722<F: Float>(t1882: F, t652: F, t621: F, t650: F, t226: F, t5270: F, t1835: F, t720: F, t1818: F, t1821: F, t219: F, t225: F, t5317: F) -> (F, F, F, F, F) {
    let t5706 = t1882 * t652;
    let t5709 = F::new(0.48245938496077605201e2) * t650 * t5706 * t621;
    let t5710 = t226 * t5270;
    let t5714 = t1835 * t5270 * t720;
    let t5717 = t1818 * t5270 * t1821;
    let t5720 = t219 * t5317 * t225;
    (t5709, t5710, t5714, t5717, t5720)
}
