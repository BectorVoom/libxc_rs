//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 824/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk824(t350: f64, t5799: f64, t1238: f64, t773: f64, t955: f64, t1227: f64, t1234: f64, t2229: f64, t342: f64, t35: f64, t360: f64, t5740: f64, t5749: f64, t5752: f64, t5755: f64, t5756: f64, t5762: f64, t5763: f64, t5766: f64, t5774: f64, t5775: f64, t5779: f64, t5785: f64, t5787: f64, t5789: f64, t5791: f64, t5795: f64, t5797: f64, t63: f64) -> (f64, f64, f64, f64) {
    let t5801 = 0.9743416666666667_f64 * t5799 * t350;
    let t5802 = t1238 * t773;
    let t5803 = t5802 * t955;
    let t5804 = 0.3247805555555556_f64 * t5803;
    let t5805 = 11.75232_f64 * t63 * t5740 * t342 + 5.87616_f64 * t63 * t2229 * t1227 + t5749 + t5752 - t5755 - 29.3808_f64 * t63 * t5756 * t1234 - t5762 - 1.46904_f64 * t63 * t5763 - 6.0_f64 * t360 * t35 * t5766 - t5774 + 3.0_f64 * t360 * t35 * t5775 + 3.0_f64 / 2.0_f64 * t360 * t35 * t5779 - t5785 - t5787 - t5789 - 2.0_f64 / 9.0_f64 * t5791 + t5795 - 0.48968_f64 * t5797 + t5801 + t5804;
    (t5801, t5802, t5804, t5805)
}
