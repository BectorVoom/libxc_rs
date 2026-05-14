//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1012/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1012<F: Float>(t1114: F, t18857: F, t345: F, t6360: F, t930: F, t951: F, t13867: F, t1680: F, t4685: F, t4719: F, t6366: F, t9804: F, t2933: F, t6390: F, t6380: F, t659: F) -> (F, F, F, F, F, F, F, F) {
    let t18858 = t1114 * t18857;
    let t18859 = t345 * t18858;
    let t18864 = t6360 * t930;
    let t18866 = 1.0 * t18864 * t951;
    let t18868 = 2.0 * t13867 * t1680;
    let t18870 = 2.0 * t4685 * t4719;
    let t18872 = 2.0 * t9804 * t6366;
    let t18874 = 1.0 * t2933 * t6390;
    let t18877 = t659 * t6380;
    (t18858, t18859, t18866, t18868, t18870, t18872, t18874, t18877)
}
