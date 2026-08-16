//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 862/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk862<F: Float>(t1114: F, t18857: F, t345: F, t6360: F, t930: F, t951: F, t13867: F, t1680: F, t4685: F, t4719: F, t6366: F, t9804: F) -> (F, F, F, F, F, F) {
    let t18858 = t1114 * t18857;
    let t18859 = t345 * t18858;
    let t18864 = t6360 * t930;
    let t18866 = F::cast_from(1.0_f64) * t18864 * t951;
    let t18868 = F::cast_from(2.0_f64) * t13867 * t1680;
    let t18870 = F::cast_from(2.0_f64) * t4685 * t4719;
    let t18872 = F::cast_from(2.0_f64) * t9804 * t6366;
    (t18858, t18859, t18866, t18868, t18870, t18872)
}
