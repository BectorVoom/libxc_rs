//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1267/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1267<F: Float>(t7968: F, t99059: F, t98794: F, t98863: F, t18221: F, t28843: F, t7978: F, t28793: F, t7974: F, t98887: F, t98918: F, t27601: F, t28714: F) -> (F, F, F, F, F, F, F, F) {
    let t99610 = F::cast_from(0.92754700520833333333e-4_f64) * t7968 * t99059;
    let t99615 = F::cast_from(0.10317654320987654321e-2_f64) * t98794;
    let t99630 = F::cast_from(0.23214722222222222222e-2_f64) * t98863;
    let t99639 = t7978 * t18221 * t28843;
    let t99644 = F::cast_from(0.61782407407407407408e-3_f64) * t28793 * t7974;
    let t99646 = F::cast_from(0.23214722222222222222e-2_f64) * t98887;
    let t99667 = F::cast_from(0.15476481481481481481e-2_f64) * t98918;
    let t99671 = F::cast_from(0.23168402777777777778e-3_f64) * t28714 * t27601;
    (t99610, t99615, t99630, t99639, t99644, t99646, t99667, t99671)
}
