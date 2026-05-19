//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1308/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1308<F: Float>(t18210: F, t28810: F, t7978: F, t99023: F, t98743: F, t61664: F, t6176: F, t7981: F, t95009: F, t95135: F, t95137: F, t98747: F, t98751: F, t98754: F, t99079: F, t99219: F) -> F {
    let t99591 = F::cast_from(0.46336805555555555556e-3_f64) * t7978 * t18210 * t28810;
    let t99593 = F::cast_from(0.23168402777777777778e-3_f64) * t7978 * t99023;
    let t99600 = F::cast_from(0.15476481481481481481e-2_f64) * t98743;
    let t99606 = F::cast_from(0.61782407407407407408e-3_f64) * t99219 * t7981 - t99591 - t99593 + F::cast_from(0.208515625e-2_f64) * t7978 * t99079 + F::cast_from(0.208515625e-2_f64) * t7978 * t6176 * t95009 * t61664 + t99600 + F::cast_from(0.34822083333333333332e-2_f64) * t98747 - F::cast_from(0.11584201388888888889e-3_f64) * t95135 - F::cast_from(0.23214722222222222222e-2_f64) * t98751 - F::cast_from(0.23214722222222222222e-2_f64) * t98754 + F::cast_from(0.15445601851851851852e-3_f64) * t95137;
    t99606
}
