//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 842/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk842<F: Float>(t350: F, t5799: F, t1238: F, t773: F, t955: F, t1227: F, t1234: F, t2229: F, t342: F, t35: F, t360: F, t5740: F, t5749: F, t5752: F, t5755: F, t5756: F, t5762: F, t5763: F, t5766: F, t5774: F, t5775: F, t5779: F, t5785: F, t5787: F, t5789: F, t5791: F, t5795: F, t5797: F, t63: F) -> (F, F, F, F, F) {
    let t5801 = F::new(0.9743416666666667) * t5799 * t350;
    let t5802 = t1238 * t773;
    let t5803 = t5802 * t955;
    let t5804 = F::new(0.3247805555555556) * t5803;
    let t5805 = F::new(11.75232) * t63 * t5740 * t342 + F::new(5.87616) * t63 * t2229 * t1227 + t5749 + t5752 - t5755 - F::new(29.3808) * t63 * t5756 * t1234 - t5762 - F::new(1.46904) * t63 * t5763 - F::new(6.0) * t360 * t35 * t5766 - t5774 + F::new(3.0) * t360 * t35 * t5775 + F::new(3.0) / F::new(2.0) * t360 * t35 * t5779 - t5785 - t5787 - t5789 - F::new(2.0) / F::new(9.0) * t5791 + t5795 - F::new(0.48968) * t5797 + t5801 + t5804;
    (t5801, t5802, t5803, t5804, t5805)
}
