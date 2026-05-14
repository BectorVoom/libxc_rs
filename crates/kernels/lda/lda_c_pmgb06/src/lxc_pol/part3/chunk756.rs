//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 756/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk756<F: Float>(t2186: F, t947: F, t410: F, t776: F, t360: F, t2233: F, t365: F, t350: F, t1271: F, t780: F, t955: F, t2210: F, t348: F, t1238: F, t773: F, t1227: F, t1234: F, t2229: F, t342: F, t35: F, t5740: F, t5749: F, t5752: F, t5755: F, t5756: F, t5762: F, t5763: F, t5766: F, t5774: F, t5775: F, t5779: F, t5785: F, t5787: F, t63: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5788 = t2186 * t947;
    let t5789 = 0.6495611111111111 * t5788;
    let t5790 = t410 * t776;
    let t5791 = t360 * t5790;
    let t5793 = t365 * t2233;
    let t5795 = 1.46904 * t5793 * t350;
    let t5796 = t1271 * t780;
    let t5797 = t5796 * t955;
    let t5799 = t348 * t2210;
    let t5801 = 0.9743416666666667 * t5799 * t350;
    let t5802 = t1238 * t773;
    let t5803 = t5802 * t955;
    let t5804 = 0.3247805555555556 * t5803;
    let t5805 = 11.75232 * t63 * t5740 * t342 + 5.87616 * t63 * t2229 * t1227 + t5749 + t5752 - t5755 - 29.3808 * t63 * t5756 * t1234 - t5762 - 1.46904 * t63 * t5763 - 6.0 * t360 * t35 * t5766 - t5774 + 3.0 * t360 * t35 * t5775 + 3.0 / 2.0 * t360 * t35 * t5779 - t5785 - t5787 - t5789 - 2.0 / 9.0 * t5791 + t5795 - 0.48968 * t5797 + t5801 + t5804;
    (t5789, t5790, t5793, t5796, t5799, t5801, t5802, t5804, t5805)
}
