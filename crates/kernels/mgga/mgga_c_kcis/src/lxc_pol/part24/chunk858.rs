//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 858/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk858<F: Float>(t330: F, t6320: F, t10324: F, t829: F, t1670: F, t1727: F, t14196: F, t934: F, t1045: F, t14170: F, t347: F, t6338: F) -> (F, F, F, F) {
    let t18773 = t6320 * t330;
    let t18775 = t10324 * t18773 * t829;
    let t18778 = t1670 * t1727;
    let t18780 = t14196 * t18778 * t934;
    let t18784 = t14170 * t18778 * t1045;
    let t18787 = t347 * t6338;
    (t18775, t18780, t18784, t18787)
}
