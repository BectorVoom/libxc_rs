//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 522/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk522(t475: f64, t9448: f64, t9438: f64, t2487: f64, t203: f64, t539: f64, t107: f64, t569: f64, t9127: f64, t568: f64, t600: f64, t1580: f64, t1641: f64, t3149: f64, t3153: f64, t3159: f64, t3173: f64, t3200: f64, t3204: f64, t4953: f64, t541: f64, t574: f64, t597: f64, t9442: f64, t9446: f64) -> (f64, f64, f64) {
    let t9449 = t9448 * t475;
    let t9450 = t9438 * t9449;
    let t9451 = t2487 * t9450;
    let t9453 = t539 * t203;
    let t9454 = t107 * t9453;
    let t9461 = t569 * t9127;
    let t9462 = t568 * t9461;
    let t9469 = t600 * t9127;
    let t9470 = t568 * t9469;
    let t9475 = 0.7988109573733489516e-2_f64 * t9442 - 0.15976219147466979032e-1_f64 * t9446 + 0.15976219147466979032e-1_f64 * t9451 - 0.7150097990370085334e0_f64 * t3159 * t9454 + 0.23833659967900284446e0_f64 * t3153 * t541 + 0.23833659967900284446e0_f64 * t3149 * t541 - 0.23005755572352449806e1_f64 * t574 * t9462 - 0.69017266717057349418e1_f64 * t4953 * t3200 + 0.23005755572352449806e1_f64 * t1580 * t3204 + 0.23005755572352449806e1_f64 * t597 * t9470 - 0.46011511144704899612e1_f64 * t1641 * t3173;
    (t9451, t9453, t9475)
}
