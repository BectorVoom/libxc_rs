//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1135/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1135(t24785: f64, t12782: f64, t5211: f64, t7106: f64, t1820: f64, t1885: f64, t41432: f64, t995: f64, t12544: f64, t7130: f64, t32670: f64, t41359: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48092 = 128.0_f64 / 1215.0_f64 * t24785;
    let t48095 = 32.0_f64 / 15.0_f64 * t5211 * t7106 * t12782;
    let t48099 = 32.0_f64 / 5.0_f64 * t1820 * t1885 * t41432 * t995;
    let t48101 = 16.0_f64 / 5.0_f64 * t7130 * t12544;
    let t48102 = 16.0_f64 / 45.0_f64 * t32670;
    let t48103 = 32.0_f64 / 15.0_f64 * t41359;
    (t48092, t48095, t48099, t48101, t48102, t48103)
}
