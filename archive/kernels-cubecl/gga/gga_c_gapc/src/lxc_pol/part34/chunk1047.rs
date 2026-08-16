//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1047/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1047<F: Float>(t11375: F, t11377: F, t11389: F, t11406: F, t11410: F, t11415: F, t11421: F, t11426: F, t11432: F, t11436: F, t11440: F, t11443: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12086 = F::cast_from(0.21103240995305505364e-7_f64) * t11375;
    let t12087 = F::cast_from(0.21103240995305505364e-7_f64) * t11377;
    let t12090 = F::cast_from(0.49522272202316919254e-5_f64) * t11389;
    let t12093 = F::cast_from(0.40483072916666666669e-4_f64) * t11406;
    let t12094 = F::cast_from(0.8433973524305555556e-6_f64) * t11410;
    let t12095 = F::cast_from(0.73797268337673611115e-6_f64) * t11415;
    let t12096 = F::cast_from(0.47342907336462418837e-4_f64) * t11421;
    let t12097 = F::cast_from(0.20241536458333333334e-3_f64) * t11426;
    let t12098 = F::cast_from(0.30775559784820528656e-8_f64) * t11432;
    let t12099 = F::cast_from(0.21720231316129303386e-4_f64) * t11436;
    let t12100 = F::cast_from(0.21720231316129303386e-4_f64) * t11440;
    let t12101 = F::cast_from(0.5686343261418565457e-6_f64) * t11443;
    (t12086, t12087, t12090, t12093, t12094, t12095, t12096, t12097, t12098, t12099, t12100, t12101)
}
