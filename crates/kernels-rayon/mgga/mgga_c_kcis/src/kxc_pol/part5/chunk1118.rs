//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1118/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1118(t1114: f64, t18857: f64, t345: f64, t6360: f64, t930: f64, t951: f64, t13867: f64, t1680: f64, t4685: f64, t4719: f64, t6366: f64, t9804: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18858 = t1114 * t18857;
    let t18859 = t345 * t18858;
    let t18864 = t6360 * t930;
    let t18866 = 1.0_f64 * t18864 * t951;
    let t18868 = 2.0_f64 * t13867 * t1680;
    let t18870 = 2.0_f64 * t4685 * t4719;
    let t18872 = 2.0_f64 * t9804 * t6366;
    (t18858, t18859, t18866, t18868, t18870, t18872)
}
