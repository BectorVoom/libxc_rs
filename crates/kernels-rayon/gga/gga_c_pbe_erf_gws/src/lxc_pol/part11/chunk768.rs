//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 768/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk768(t12537: f64, t1661: f64, t587: f64, t1010: f64, t10843: f64, t1017: f64, t10365: f64, t1885: f64, t1820: f64, t2615: f64, t3527: f64, t12345: f64, t591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12538 = t1661 * t12537;
    let t12540 = 8.0_f64 / 9.0_f64 * t587 * t12538;
    let t12542 = 8.0_f64 / 15.0_f64 * t10843 * t1010;
    let t12543 = t10365 * t1017;
    let t12544 = t1885 * t12543;
    let t12546 = 4.0_f64 / 5.0_f64 * t1820 * t12544;
    let t12548 = 4.0_f64 / 15.0_f64 * t2615 * t3527;
    let t12549 = t591 * t12345;
    (t12538, t12540, t12542, t12543, t12544, t12546, t12548, t12549)
}
