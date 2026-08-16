//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3196/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3196<F: Float>(t1227: F, t13969: F, t19067: F, t1222: F, t18297: F, t18982: F, t18947: F, t3506: F, t11719: F, t18302: F, t11546: F, t1174: F, t11814: F, t15625: F, t15761: F, t18300: F, t3440: F, t3507: F, t44725: F, t44863: F, t44906: F, t45030: F, t45178: F, t4582: F, t5005: F, t53267: F, t6221: F, t63378: F, t63386: F, t63394: F) -> F {
    let t66406 = t1227 * t13969 * t19067;
    let t66408 = t18297 * t1222;
    let t66410 = t18982 * t1222;
    let t66413 = t3506 * t13969 * t18947;
    let t66437 = t11719 * t13969 * t18302;
    let t66442 = -t5005 * t15761 / F::cast_from(2304.0_f64) + t11814 * t6221 / F::cast_from(3072.0_f64) + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t66406 - t66408 / F::cast_from(216.0_f64) + F::cast_from(19.0_f64) / F::cast_from(1296.0_f64) * t66410 + t66413 / F::cast_from(576.0_f64) + t1174 * t3440 * t63394 / F::cast_from(108.0_f64) + t1174 * t3440 * t63386 / F::cast_from(36.0_f64) + t11719 * t4582 * t18300 * t44906 / F::cast_from(512.0_f64) + t44863 * t4582 * t18300 * t44725 * t3507 / F::cast_from(128.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t45030 * t4582 * t18300 * t15625 + t45178 / F::cast_from(648.0_f64) - t53267 / F::cast_from(3456.0_f64) + t66437 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(54.0_f64) * t1174 * t11546 * t63378;
    t66442
}
