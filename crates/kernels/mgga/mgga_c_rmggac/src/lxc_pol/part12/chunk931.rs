//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 931/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk931<F: Float>(t7508: F, t8568: F, t27102: F, t3351: F, t3352: F, t875: F, t2286: F, t34881: F, t7424: F, t8571: F, t2868: F, t39994: F, t39998: F, t40002: F, t40007: F, t40012: F, t40015: F, t40018: F, t40021: F, t40024: F, t40027: F, t40032: F, t40037: F, t7571: F) -> F {
    let t40039 = t7508 * t8568;
    let t40043 = t3351 * t3352 * t875 * t27102;
    let t40045 = t34881 * t2286;
    let t40047 = t8571 * t7424;
    let t40049 = F::new(0.8980681276397856423e-1) * t39994 + t39998 + F::new(0.1064114997332445985e-4) * t40002 - F::new(0.25538759935978703638e-4) * t40007 - F::new(0.59871208509319042821e-1) * t2868 * t7571 - F::new(0.16364796992547205037e0) * t40012 - F::new(0.40911992481368012592e-1) * t40015 + F::new(0.81823984962736025184e-1) * t40018 + F::new(0.5987120850931904282e-1) * t40021 - F::new(0.8980681276397856423e-1) * t40024 - F::new(0.5987120850931904282e-1) * t40027 - F::new(0.25538759935978703638e-4) * t40032 - F::new(0.31923449919973379548e-4) * t40037 - F::new(0.68186654135613354322e-2) * t40039 + F::new(0.25538759935978703638e-4) * t40043 - F::new(0.59590439850616975156e-4) * t40045 + F::new(0.25538759935978703638e-4) * t40047;
    t40049
}
