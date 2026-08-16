//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 688/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk688(t5536: f64, t587: f64, t1759: f64, t562: f64, t2559: f64, t1820: f64, t1661: f64, t597: f64, t610: f64, t1802: f64, t590: f64, t1804: f64, t418: f64, t572: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5538 = 4.0_f64 / 9.0_f64 * t587 * t5536;
    let t5539 = t1759 * t562;
    let t5540 = t2559 * t5539;
    let t5542 = 8.0_f64 / 9.0_f64 * t1820 * t5540;
    let t5543 = t1661 * t597;
    let t5544 = t1759 * t610;
    let t5545 = t5543 * t5544;
    let t5547 = 4.0_f64 / 9.0_f64 * t587 * t5545;
    let t5548 = t590 * t1802;
    let t5550 = t1804 * t572 * t418;
    (t5538, t5539, t5540, t5542, t5543, t5544, t5545, t5547, t5548, t5550)
}
