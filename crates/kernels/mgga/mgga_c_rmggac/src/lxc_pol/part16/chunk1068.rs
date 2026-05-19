//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1068/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1068<F: Float>(t37201: F, t37202: F, t37203: F, t37214: F, t42712: F, t42714: F, t42715: F, t44891: F, t44894: F, t44901: F, t44906: F, t44909: F, t44911: F, t44916: F, t44920: F, t44925: F, t44929: F) -> F {
    let t48237 = t37201 + t37202 - t37203 + F::cast_from(0.1440846329149835838e-2_f64) * t44891 - F::cast_from(0.20496175532535769482e-3_f64) * t44894 + t42712 - t42714 - t42715 - F::cast_from(0.638468998399467591e-4_f64) * t44901 - F::cast_from(0.638468998399467591e-4_f64) * t44906 - F::cast_from(0.19863479950205658386e-4_f64) * t44909 - F::cast_from(0.5454932330849068346e-1_f64) * t44911 + F::cast_from(0.30487649791575028312e-3_f64) * t44916 + F::cast_from(0.30487649791575028312e-3_f64) * t44920 + F::cast_from(0.60975299583150056624e-3_f64) * t44925 + F::cast_from(0.60975299583150056624e-3_f64) * t44929 - t37214;
    t48237
}
