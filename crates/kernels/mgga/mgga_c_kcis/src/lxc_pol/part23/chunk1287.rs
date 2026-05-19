//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1287/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1287<F: Float>(t1370: F, t7984: F, t98057: F, t18210: F, t28737: F, t7968: F, t28707: F, t27595: F, t27607: F, t28741: F, t18159: F, t27583: F, t27598: F, t27665: F, t28727: F, t28853: F, t98052: F, t98055: F, t98069: F, t99043: F) -> (F, F, F) {
    let t99046 = t1370 * t7984;
    let t99052 = F::cast_from(0.15476481481481481481e-2_f64) * t98057;
    let t99056 = t18210 * t28737;
    let t99058 = F::cast_from(0.30918233506944444444e-4_f64) * t7968 * t99056;
    let t99059 = t18210 * t28707;
    let t99060 = t27595 * t99059;
    let t99065 = F::cast_from(0.7722800925925925926e-4_f64) * t27607 * t28741;
    let t99066 = -F::cast_from(0.23168402777777777778e-3_f64) * t27583 * t99043 - F::cast_from(0.46336805555555555556e-3_f64) * t27583 * t99046 * t18159 - F::cast_from(0.30952962962962962962e-2_f64) * t98052 - F::cast_from(0.51588271604938271603e-2_f64) * t98055 - t99052 + F::cast_from(0.37101880208333333334e-3_f64) * t28853 * t27598 + F::cast_from(0.34822083333333333332e-2_f64) * t98069 - t99058 - F::cast_from(0.61890573922526041667e-5_f64) * t99060 - F::cast_from(0.61782407407407407408e-3_f64) * t28727 * t27665 - t99065;
    (t99056, t99059, t99066)
}
