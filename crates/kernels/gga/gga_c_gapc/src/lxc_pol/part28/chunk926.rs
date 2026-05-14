//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 926/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk926<F: Float>(t11375: F, t11377: F, t11389: F, t11406: F, t11410: F, t11415: F, t11421: F, t11426: F, t11432: F, t11436: F, t11440: F, t11443: F, t11459: F, t11373: F, t11382: F, t11385: F, t11392: F, t11403: F, t11445: F, t11453: F) -> (F,) {
    let t12086 = 0.21103240995305505364e-7 * t11375;
    let t12087 = 0.21103240995305505364e-7 * t11377;
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
    let t12105 = -0.90579542097823505425e-7 * t11373 + t12086 + t12087 - 0.4419852458519115466e-8 * t11382 - 0.66297786877786731988e-7 * t11385 + t12090 + 0.57970906942607043474e-5 * t11392 - 0.14340192936791314022e-8 * t11403 + t12093 + t12094 - t12095 - t12096 - t12097 + t12098 - t12099 - t12100 - t12101 - 0.64087860648527174255e-6 * t11445 + 0.98332751566569010434e-8 * t11453 + t12104;
    (t12105,)
}
