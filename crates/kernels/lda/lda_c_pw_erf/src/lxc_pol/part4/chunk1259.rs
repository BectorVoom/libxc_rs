//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1259/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1259<F: Float>(t1234: F, t2443: F, t1386: F, t6597: F, t14360: F, t5031: F, t822: F, t1960: F, t2067: F, t14365: F, t18694: F, t18696: F, t18697: F, t18698: F, t18702: F, t18703: F, t18704: F, t18705: F, t18706: F, t18707: F, t18709: F) -> (F, F, F, F, F, F, F) {
    let t18710 = t2443 * t1234;
    let t18711 = 8.0 / 45.0 * t18710;
    let t18712 = t6597 * t1386;
    let t18713 = 16.0 / 45.0 * t18712;
    let t18714 = 16.0 / 45.0 * t14360;
    let t18716 = 4.0 / 15.0 * t822 * t5031;
    let t18718 = 8.0 / 15.0 * t1960 * t2067;
    let t18719 = 16.0 / 135.0 * t14365;
    let t18720 = t18694 + t18696 + t18697 - t18698 - t18702 - t18703 - t18704 - t18705 + t18706 - t18707 - t18709 - t18711 + t18713 - t18714 - t18716 - t18718 + t18719;
    (t18711, t18713, t18714, t18716, t18718, t18719, t18720)
}
