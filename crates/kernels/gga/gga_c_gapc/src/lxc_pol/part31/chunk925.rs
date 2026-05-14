//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 925/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk925<F: Float>(t11389: F, t11406: F, t11410: F, t11415: F, t11421: F, t11426: F, t11432: F, t11436: F, t11440: F, t11443: F, t11459: F, t11466: F, t11469: F, t11471: F, t11475: F, t11477: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12090 = 0.49522272202316919254e-5 * t11389;
    let t12093 = 0.40483072916666666669e-4 * t11406;
    let t12094 = 0.8433973524305555556e-6 * t11410;
    let t12095 = 0.73797268337673611115e-6 * t11415;
    let t12096 = 0.47342907336462418837e-4 * t11421;
    let t12097 = 0.20241536458333333334e-3 * t11426;
    let t12098 = 0.30775559784820528656e-8 * t11432;
    let t12099 = 0.21720231316129303386e-4 * t11436;
    let t12100 = 0.21720231316129303386e-4 * t11440;
    let t12101 = 0.5686343261418565457e-6 * t11443;
    let t12104 = 0.10110318318802209383e-5 * t11459;
    let t12107 = 0.31675337336021900771e-5 * t11466;
    let t12108 = 0.67530371184977617164e-6 * t11469;
    let t12109 = 0.67530371184977617164e-6 * t11471;
    let t12110 = 0.40022999988963401107e-7 * t11475;
    let t12111 = 0.40096157891080460192e-6 * t11477;
    (t12090, t12093, t12094, t12095, t12096, t12097, t12098, t12099, t12100, t12101, t12104, t12107, t12108, t12109, t12110, t12111)
}
