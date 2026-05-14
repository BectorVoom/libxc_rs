//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 604/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk604<F: Float>(t1325: F, t3819: F, t1310: F, t1472: F, t1360: F, t593: F, t1308: F, t571: F, t1381: F, t559: F, t1485: F, t581: F, t1352: F, t1356: F, t549: F, t1319: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3821 = 8.0 / 15.0 * t1325 * t3819;
    let t3823 = 8.0 / 15.0 * t1472 * t1310;
    let t3824 = t1360 * t593;
    let t3825 = t1308 * t3824;
    let t3827 = 4.0 / 15.0 * t571 * t3825;
    let t3828 = t559 * t1381;
    let t3829 = t1308 * t3828;
    let t3831 = 4.0 / 15.0 * t571 * t3829;
    let t3832 = t1485 * t581;
    let t3833 = t1352 * t593;
    let t3834 = t3832 * t3833;
    let t3836 = 4.0 / 9.0 * t571 * t3834;
    let t3837 = t1356 * t549;
    let t3838 = t1319 * t3837;
    (t3821, t3823, t3824, t3825, t3827, t3828, t3829, t3831, t3832, t3833, t3834, t3836, t3837, t3838)
}
