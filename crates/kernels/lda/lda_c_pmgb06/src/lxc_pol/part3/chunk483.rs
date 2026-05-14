//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 483/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk483<F: Float>(t2010: F, t2012: F, t435: F, t823: F, t132: F, t489: F, t852: F, t161: F, t1798: F, t205: F, t208: F, t579: F, t794: F, t213: F, t871: F, t97: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2014 = 2.0 / 45.0 * t2010 * t2012;
    let t2015 = t435 * t823;
    let t2016 = t132 * t2015;
    let t2017 = t2016 / 45.0;
    let t2018 = t489 * t852;
    let t2019 = t161 * t2018;
    let t2020 = t2019 / 45.0;
    let t2021 = t1798 * t205;
    let t2022 = t2021 * t208;
    let t2025 = t794 * t579;
    let t2026 = t2025 * t208;
    let t2027 = t2026 * t213;
    let t2029 = t871 * t97;
    (t2014, t2015, t2017, t2018, t2020, t2021, t2022, t2025, t2026, t2027, t2029)
}
