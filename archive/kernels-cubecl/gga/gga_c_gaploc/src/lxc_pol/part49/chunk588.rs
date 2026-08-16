//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 588/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk588<F: Float>(t1628: F, t3303: F, t3314: F, t3299: F, t1589: F, t3255: F, t3290: F, t3251: F, t7403: F, t959: F, t7340: F, t2049: F, t2087: F, t2103: F, t3271: F, t3277: F, t3284: F, t3287: F, t5771: F, t5775: F, t784: F, t797: F, t813: F, t833: F) -> (F, F, F) {
    let t9908 = t1628 * t3303;
    let t9911 = t1628 * t3314;
    let t9914 = t1628 * t3299;
    let t9917 = t1589 * t3255;
    let t9920 = t1628 * t3290;
    let t9925 = t1589 * t3251;
    let t9935 = F::cast_from(0.29792074959875355558e-1_f64) * t7403 * t959;
    let t9937 = F::cast_from(0.29792074959875355558e-1_f64) * t7340 * t959;
    let t9938 = -F::cast_from(0.30674340763136599741e1_f64) * t813 * t9908 - F::cast_from(0.92023022289409799224e1_f64) * t2087 * t9911 + F::cast_from(0.15337170381568299871e2_f64) * t833 * t9914 - F::cast_from(0.23833659967900284446e0_f64) * t797 * t9917 - F::cast_from(0.61348681526273199483e1_f64) * t813 * t9920 - F::cast_from(0.7150097990370085334e0_f64) * t3277 * t5775 + F::cast_from(0.47667319935800568892e0_f64) * t2103 * t9925 + F::cast_from(0.23833659967900284446e0_f64) * t3271 * t784 + F::cast_from(0.71500979903700853338e0_f64) * t5771 * t3284 - F::cast_from(0.35750489951850426669e0_f64) * t2049 * t3287 + t9935 + t9937;
    (t9935, t9937, t9938)
}
