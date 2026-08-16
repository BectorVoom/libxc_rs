//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1049/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1049(t11375: f64, t11377: f64, t11389: f64, t11406: f64, t11410: f64, t11415: f64, t11421: f64, t11426: f64, t11432: f64, t11436: f64, t11440: f64, t11443: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12086 = 0.21103240995305505364e-7_f64 * t11375;
    let t12087 = 0.21103240995305505364e-7_f64 * t11377;
    let t12090 = 0.49522272202316919254e-5_f64 * t11389;
    let t12093 = 0.40483072916666666669e-4_f64 * t11406;
    let t12094 = 0.8433973524305555556e-6_f64 * t11410;
    let t12095 = 0.73797268337673611115e-6_f64 * t11415;
    let t12096 = 0.47342907336462418837e-4_f64 * t11421;
    let t12097 = 0.20241536458333333334e-3_f64 * t11426;
    let t12098 = 0.30775559784820528656e-8_f64 * t11432;
    let t12099 = 0.21720231316129303386e-4_f64 * t11436;
    let t12100 = 0.21720231316129303386e-4_f64 * t11440;
    let t12101 = 0.5686343261418565457e-6_f64 * t11443;
    (t12086, t12087, t12090, t12093, t12094, t12095, t12096, t12097, t12098, t12099, t12100, t12101)
}
