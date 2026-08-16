//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1021/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1021(t10685: f64, t2615: f64, t12518: f64, t586: f64, t12716: f64, t5129: f64, t587: f64, t11022: f64, t2612: f64, t1022: f64, t3553: f64, t12626: f64, t1620: f64, t7877: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41633 = t2615 * t10685;
    let t41638 = t12518 * t586;
    let t41666 = t587 * t5129 * t12716;
    let t41668 = t2612 * t11022;
    let t41690 = t1022 * t3553;
    let t41702 = t1620 * t7877 * t12626;
    (t41633, t41638, t41666, t41668, t41690, t41702)
}
