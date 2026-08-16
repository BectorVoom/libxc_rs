//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 846/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk846(t155: f64, t8141: f64, t2205: f64, t568: f64, t205: f64, t200: f64, t7991: f64, t2193: f64, t727: f64, t1067: f64, t2183: f64, t3317: f64, t3333: f64, t3335: f64, t3340: f64, t3342: f64, t7801: f64, t7805: f64, t7809: f64, t7811: f64, t7814: f64, t7817: f64, t7834: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8679 = t155 * t8141;
    let t8681 = t568 * t2205;
    let t8682 = t205 * t8681;
    let t8684 = t200 * t8141;
    let t8686 = t200 * t7991;
    let t8689 = t727 * t568 * t2193;
    let t8691 = t2183 * t1067;
    let t8705 = -7.919542066025344_f64 * t7801 - 11.879313099038017_f64 * t7805 - 11.879313099038017_f64 * t7809 - 11.879313099038017_f64 * t7811 - 11.879313099038017_f64 * t7814 - 11.879313099038017_f64 * t7817 - 11.879313099038017_f64 * t7834 - 11.879313099038017_f64 * t3335 - 7.919542066025344_f64 * t3342 + t3333 - t3340 + 11.879313099038017_f64 * t3317;
    (t8679, t8682, t8684, t8686, t8689, t8691, t8705)
}
