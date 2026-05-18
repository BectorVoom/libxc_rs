//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 846/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk846<F: Float>(t155: F, t8141: F, t2205: F, t568: F, t205: F, t200: F, t7991: F, t2193: F, t727: F, t1067: F, t2183: F, t3317: F, t3333: F, t3335: F, t3340: F, t3342: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F) -> (F, F, F, F, F, F, F) {
    let t8679 = t155 * t8141;
    let t8681 = t568 * t2205;
    let t8682 = t205 * t8681;
    let t8684 = t200 * t8141;
    let t8686 = t200 * t7991;
    let t8689 = t727 * t568 * t2193;
    let t8691 = t2183 * t1067;
    let t8705 = -F::new(7.919542066025344) * t7801 - F::new(11.879313099038017) * t7805 - F::new(11.879313099038017) * t7809 - F::new(11.879313099038017) * t7811 - F::new(11.879313099038017) * t7814 - F::new(11.879313099038017) * t7817 - F::new(11.879313099038017) * t7834 - F::new(11.879313099038017) * t3335 - F::new(7.919542066025344) * t3342 + t3333 - t3340 + F::new(11.879313099038017) * t3317;
    (t8679, t8682, t8684, t8686, t8689, t8691, t8705)
}
