//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 588/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk588(t1628: f64, t3303: f64, t3314: f64, t3299: f64, t1589: f64, t3255: f64, t3290: f64, t3251: f64, t7403: f64, t959: f64, t7340: f64, t2049: f64, t2087: f64, t2103: f64, t3271: f64, t3277: f64, t3284: f64, t3287: f64, t5771: f64, t5775: f64, t784: f64, t797: f64, t813: f64, t833: f64) -> (f64, f64, f64) {
    let t9908 = t1628 * t3303;
    let t9911 = t1628 * t3314;
    let t9914 = t1628 * t3299;
    let t9917 = t1589 * t3255;
    let t9920 = t1628 * t3290;
    let t9925 = t1589 * t3251;
    let t9935 = 0.29792074959875355558e-1_f64 * t7403 * t959;
    let t9937 = 0.29792074959875355558e-1_f64 * t7340 * t959;
    let t9938 = -0.30674340763136599741e1_f64 * t813 * t9908 - 0.92023022289409799224e1_f64 * t2087 * t9911 + 0.15337170381568299871e2_f64 * t833 * t9914 - 0.23833659967900284446e0_f64 * t797 * t9917 - 0.61348681526273199483e1_f64 * t813 * t9920 - 0.7150097990370085334e0_f64 * t3277 * t5775 + 0.47667319935800568892e0_f64 * t2103 * t9925 + 0.23833659967900284446e0_f64 * t3271 * t784 + 0.71500979903700853338e0_f64 * t5771 * t3284 - 0.35750489951850426669e0_f64 * t2049 * t3287 + t9935 + t9937;
    (t9935, t9937, t9938)
}
