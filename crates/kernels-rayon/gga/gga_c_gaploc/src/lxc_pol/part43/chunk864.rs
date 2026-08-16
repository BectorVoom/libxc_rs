//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 864/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk864(t26435: f64, t6710: f64, t9438: f64, t9060: f64, t986: f64, t1415: f64, t1646: f64, t12990: f64, t7007: f64, t30733: f64, t27003: f64, t587: f64) -> (f64, f64, f64, f64, f64) {
    let t42400 = t6710 * t9438 * t26435;
    let t42401 = 0.15976219147466979032e-1_f64 * t42400;
    let t42402 = t9060 * t986;
    let t42405 = 0.35750489951850426669e0_f64 * t1415 * t42402 * t1646;
    let t42407 = 0.71500979903700853338e0_f64 * t12990 * t7007;
    let t42412 = t12990 * t30733;
    let t42413 = 0.59584149919750711116e-1_f64 * t42412;
    let t42420 = t587 * t9438 * t27003;
    (t42401, t42405, t42407, t42413, t42420)
}
