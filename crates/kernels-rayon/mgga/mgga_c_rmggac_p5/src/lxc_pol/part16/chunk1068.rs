//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1068/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1068(t37201: f64, t37202: f64, t37203: f64, t37214: f64, t42712: f64, t42714: f64, t42715: f64, t44891: f64, t44894: f64, t44901: f64, t44906: f64, t44909: f64, t44911: f64, t44916: f64, t44920: f64, t44925: f64, t44929: f64) -> f64 {
    let t48237 = t37201 + t37202 - t37203 + 0.1440846329149835838e-2_f64 * t44891 - 0.20496175532535769482e-3_f64 * t44894 + t42712 - t42714 - t42715 - 0.638468998399467591e-4_f64 * t44901 - 0.638468998399467591e-4_f64 * t44906 - 0.19863479950205658386e-4_f64 * t44909 - 0.5454932330849068346e-1_f64 * t44911 + 0.30487649791575028312e-3_f64 * t44916 + 0.30487649791575028312e-3_f64 * t44920 + 0.60975299583150056624e-3_f64 * t44925 + 0.60975299583150056624e-3_f64 * t44929 - t37214;
    t48237
}
