//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1055/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1055<F: Float>(t16962: F, t3754: F, t1369: F, t1377: F, t1444: F, t25: F, t5733: F, t493: F, t11425: F, t556: F, t1404: F, t4035: F) -> (F, F, F, F, F) {
    let t16963 = t16962 * t3754;
    let t16968 = t1369 * t1377;
    let t16969 = t16968 * t1444;
    let t16979 = t25 * t5733;
    let t16981 = t493 * t16979 / F::cast_from(144.0_f64);
    let t17009 = t556 * t11425;
    let t17019 = t1404 * t4035;
    (t16963, t16969, t16981, t17009, t17019)
}
