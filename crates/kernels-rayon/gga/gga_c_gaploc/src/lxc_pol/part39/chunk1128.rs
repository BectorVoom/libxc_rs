//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1128/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1128(t12207: f64, t2714: f64, t13861: f64, t2103: f64, t4673: f64, t43750: f64, t43752: f64, t43754: f64, t43757: f64, t43759: f64, t47357: f64, t47360: f64, t47362: f64, t47364: f64, t47366: f64) -> f64 {
    let t47368 = t2714 * t12207;
    let t47371 = t2103 * t4673 * t13861;
    let t47373 = -t43750 - t43752 - 0.11502877786176224903e2_f64 * t47357 + 0.27606906686822939767e2_f64 * t47360 + t43754 - 0.7150097990370085334e0_f64 * t47362 + t43757 + 0.35750489951850426669e0_f64 * t47364 + 0.35750489951850426669e0_f64 * t47366 + 0.35750489951850426669e0_f64 * t47368 + 0.47667319935800568892e0_f64 * t47371 - t43759;
    t47373
}
