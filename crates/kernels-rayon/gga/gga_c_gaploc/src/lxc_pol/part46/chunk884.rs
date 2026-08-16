//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 884/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk884(t12868: f64, t1580: f64, t12806: f64, t1562: f64, t4614: f64, t10533: f64, t20796: f64, t41738: f64, t26435: f64, t6710: f64, t9438: f64, t9060: f64, t986: f64) -> (f64, f64, f64, f64, f64) {
    let t42392 = 0.11502877786176224903e2_f64 * t1580 * t12868;
    let t42395 = 0.82820720060468819301e2_f64 * t1562 * t4614 * t12806;
    let t42398 = 0.27606906686822939767e2_f64 * t20796 * t10533 * t41738;
    let t42400 = t6710 * t9438 * t26435;
    let t42401 = 0.15976219147466979032e-1_f64 * t42400;
    let t42402 = t9060 * t986;
    (t42392, t42395, t42398, t42401, t42402)
}
