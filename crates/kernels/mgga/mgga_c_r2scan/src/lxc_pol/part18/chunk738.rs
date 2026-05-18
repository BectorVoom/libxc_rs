//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 738/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk738<F: Float>(t122: F, t6100: F, t261: F, t277: F, t254: F, t2132: F, t2195: F, t1598: F, t2120: F, t524: F, t1569: F, t481: F) -> (F, F, F, F, F) {
    let t6101 = t122 * t6100;
    let t6103 = t261 * t6101 * t277;
    let t6105 = F::new(0.19776387377308997907e1) * t254 * t6103;
    let t6106 = t2195 * t2132;
    let t6118 = t524 * t1598 * t2120;
    let t6121 = t1569 * t481;
    (t6101, t6105, t6106, t6118, t6121)
}
