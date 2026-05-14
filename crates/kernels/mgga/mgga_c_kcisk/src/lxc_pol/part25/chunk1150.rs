//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1150/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1150<F: Float>(t1791: F, t4597: F, t6667: F, t7234: F, t2464: F, t1636: F, t5015: F, t7283: F, t7242: F, t1785: F, t2063: F, t33059: F, t1849: F, t17182: F, t9921: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t34016 = t1791 * t4597;
    let t34017 = t34016 * t6667;
    let t34018 = t7234 * t34017;
    let t34021 = t1791 * t2464;
    let t34022 = t34021 * t1636;
    let t34023 = t5015 * t34022;
    let t34026 = t7283 * t1636;
    let t34027 = t7242 * t34026;
    let t34030 = t2063 * t1785;
    let t34031 = t33059 * t34030;
    let t34032 = t5015 * t34031;
    let t34037 = t1791 * t1849;
    let t34038 = t34037 * t6667;
    let t34039 = t7242 * t34038;
    let t34045 = t17182 * t9921;
    (t34016, t34017, t34018, t34021, t34022, t34023, t34026, t34027, t34031, t34032, t34037, t34038, t34039, t34045)
}
