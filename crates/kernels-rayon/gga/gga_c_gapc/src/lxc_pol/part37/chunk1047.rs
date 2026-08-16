//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1047/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1047(t11353: f64, t11358: f64, t11369: f64, t11375: f64, t11377: f64, t11389: f64, t11406: f64, t11410: f64, t11415: f64, t11421: f64, t11426: f64, t11432: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12079 = 0.10821235962619981449e-3_f64 * t11353;
    let t12080 = 0.42206481990611010728e-7_f64 * t11358;
    let t12083 = 0.13259557375557346398e-6_f64 * t11369;
    let t12086 = 0.21103240995305505364e-7_f64 * t11375;
    let t12087 = 0.21103240995305505364e-7_f64 * t11377;
    let t12090 = 0.49522272202316919254e-5_f64 * t11389;
    let t12093 = 0.40483072916666666669e-4_f64 * t11406;
    let t12094 = 0.8433973524305555556e-6_f64 * t11410;
    let t12095 = 0.73797268337673611115e-6_f64 * t11415;
    let t12096 = 0.47342907336462418837e-4_f64 * t11421;
    let t12097 = 0.20241536458333333334e-3_f64 * t11426;
    let t12098 = 0.30775559784820528656e-8_f64 * t11432;
    (t12079, t12080, t12083, t12086, t12087, t12090, t12093, t12094, t12095, t12096, t12097, t12098)
}
