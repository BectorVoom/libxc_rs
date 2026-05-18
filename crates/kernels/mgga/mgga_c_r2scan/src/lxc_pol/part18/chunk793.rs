//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 793/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk793<F: Float>(t2483: F, t697: F, t1721: F, t898: F, t5393: F, t5: F, t736: F, t1754: F, t2788: F, t2782: F, t584: F, t591: F) -> (F, F, F, F, F, F) {
    let t7730 = F::new(0.1301229756036208781e0) * t2483 * t697;
    let t7737 = t898 * t1721;
    let t7739 = F::new(48.0) * t5393;
    let t7741 = t2483 * t5;
    let t7743 = F::new(0.10843581300301739842e-1) * t7741 * t736;
    let t7745 = t2788 * t1754;
    let t7751 = F::new(0.1143056e0) * t584 * t2782 * t591;
    (t7730, t7737, t7739, t7743, t7745, t7751)
}
