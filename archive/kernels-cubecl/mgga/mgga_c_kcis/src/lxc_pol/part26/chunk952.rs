//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 952/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk952<F: Float>(t22035: F, t542: F, t1098: F, t7250: F, t1102: F, t15994: F, t16001: F, t16003: F, t16038: F, t21983: F, t21987: F, t21990: F, t21993: F, t21996: F, t22001: F, t22006: F, t22011: F, t22015: F, t22018: F, t22021: F, t22025: F, t22029: F, t22032: F, t344: F, t4587: F) -> F {
    let t22036 = t542 * t22035;
    let t22039 = t1098 * t7250;
    let t22043 = F::cast_from(0.16426074166666666666e-2_f64) * t1102 * t21983 - F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t21987 - F::cast_from(0.65704296666666666666e-2_f64) * t1102 * t21990 + F::cast_from(0.29201909629629629629e-2_f64) * t1102 * t21993 - F::cast_from(0.43802864444444444444e-2_f64) * t4587 * t21996 - F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t22001 + F::cast_from(0.98556445e-3_f64) * t1102 * t22006 + F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t22011 + F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t22015 + F::cast_from(0.39422577999999999999e-2_f64) * t1102 * t22018 + F::cast_from(0.52563437333333333332e-2_f64) * t4587 * t22021 + F::cast_from(0.98556445e-3_f64) * t1102 * t22025 - F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t22029 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t22032 - F::cast_from(0.98556445e-3_f64) * t344 * t22036 - F::cast_from(0.65704296666666666667e-3_f64) * t22039 - F::cast_from(0.17521145777777777778e-2_f64) * t15994 + t16001 - t16003 - F::cast_from(0.2920190962962962963e-3_f64) * t16038;
    t22043
}
