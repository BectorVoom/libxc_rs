//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1083/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1083<F: Float>(t2438: F, t925: F, t2434: F, t10762: F, t4: F, t6: F, t1251: F, t35: F, t503: F, t11709: F, t11711: F, t11713: F, t11715: F, t11751: F, t11753: F, t11755: F, t11770: F, t11775: F) -> (F, F, F, F, F, F) {
    let t15777 = t925 * t2438;
    let t15779 = t925 * t2434;
    let t15782 = t4 * t6 * t10762;
    let t15783 = t1251 * t35;
    let t15785 = t15782 * t503 * t15783;
    let t15787 = -0.006717037037037037 * t11709 - 0.0008396296296296296 * t11711 - 0.0013993827160493828 * t11713 - 0.0033585185185185185 * t11715 + 0.002518888888888889 * t11751 + 0.002239012345679012 * t11753 + 0.005037777777777778 * t11755 + 0.010075555555555556 * t11770 - 0.007556666666666666 * t11775 + 0.0008396296296296296 * t15777 - 0.0016792592592592592 * t15779 - 0.030226666666666666 * t15785;
    (t15777, t15779, t15782, t15783, t15785, t15787)
}
