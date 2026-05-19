//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1364/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1364<F: Float>(t96005: F, t1646: F, t28110: F, t3616: F, t5310: F, t96018: F, t13173: F, t15231: F, t2192: F, t2197: F, t26960: F, t26961: F, t28123: F, t3515: F, t3611: F, t46577: F, t92964: F, t92976: F, t92981: F, t96000: F, t96003: F, t96021: F) -> (F, F) {
    let t97173 = F::cast_from(0.15476481481481481481e-2_f64) * t96005;
    let t97188 = t5310 * t28110 * t1646 * t3616;
    let t97193 = F::cast_from(0.23214722222222222222e-2_f64) * t96018;
    let t97195 = F::cast_from(0.38691203703703703703e-3_f64) * t96000 + F::cast_from(0.11607361111111111111e-2_f64) * t96003 + t97173 - F::cast_from(0.34752604166666666667e-3_f64) * t46577 * t2192 * t2197 - t92964 - F::cast_from(0.61782407407407407408e-3_f64) * t26960 * t15231 * t28123 * t13173 + F::cast_from(0.11584201388888888889e-3_f64) * t26960 * t3515 * t26961 * t1646 * t3611 + F::cast_from(0.11584201388888888889e-3_f64) * t26960 * t97188 - F::cast_from(0.11584201388888888889e-3_f64) * t92976 - F::cast_from(0.41270617283950617284e-2_f64) * t92981 - t97193 + F::cast_from(0.92858888888888888886e-2_f64) * t96021;
    (t97188, t97195)
}
