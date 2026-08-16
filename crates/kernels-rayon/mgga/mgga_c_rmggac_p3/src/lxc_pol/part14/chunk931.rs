//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 931/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk931(t7508: f64, t8568: f64, t27102: f64, t3351: f64, t3352: f64, t875: f64, t2286: f64, t34881: f64, t7424: f64, t8571: f64, t2868: f64, t39994: f64, t39998: f64, t40002: f64, t40007: f64, t40012: f64, t40015: f64, t40018: f64, t40021: f64, t40024: f64, t40027: f64, t40032: f64, t40037: f64, t7571: f64) -> f64 {
    let t40039 = t7508 * t8568;
    let t40043 = t3351 * t3352 * t875 * t27102;
    let t40045 = t34881 * t2286;
    let t40047 = t8571 * t7424;
    let t40049 = 0.8980681276397856423e-1_f64 * t39994 + t39998 + 0.1064114997332445985e-4_f64 * t40002 - 0.25538759935978703638e-4_f64 * t40007 - 0.59871208509319042821e-1_f64 * t2868 * t7571 - 0.16364796992547205037e0_f64 * t40012 - 0.40911992481368012592e-1_f64 * t40015 + 0.81823984962736025184e-1_f64 * t40018 + 0.5987120850931904282e-1_f64 * t40021 - 0.8980681276397856423e-1_f64 * t40024 - 0.5987120850931904282e-1_f64 * t40027 - 0.25538759935978703638e-4_f64 * t40032 - 0.31923449919973379548e-4_f64 * t40037 - 0.68186654135613354322e-2_f64 * t40039 + 0.25538759935978703638e-4_f64 * t40043 - 0.59590439850616975156e-4_f64 * t40045 + 0.25538759935978703638e-4_f64 * t40047;
    t40049
}
