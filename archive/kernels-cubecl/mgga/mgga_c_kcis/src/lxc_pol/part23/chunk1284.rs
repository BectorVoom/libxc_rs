//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1284/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1284<F: Float>(t17326: F, t27494: F, t12345: F, t4310: F, t8186: F, t2104: F, t30409: F, t4312: F, t6176: F, t97997: F, t27563: F, t28727: F) -> (F, F, F, F, F) {
    let t98959 = F::cast_from(2.0_f64) * t27494 * t17326;
    let t98963 = F::cast_from(6.0_f64) * t12345 * t8186 * t4310;
    let t98971 = t6176 * t30409 * t2104 * t4312;
    let t98978 = F::cast_from(0.15476481481481481481e-2_f64) * t97997;
    let t98986 = F::cast_from(0.61782407407407407408e-3_f64) * t28727 * t27563;
    (t98959, t98963, t98971, t98978, t98986)
}
