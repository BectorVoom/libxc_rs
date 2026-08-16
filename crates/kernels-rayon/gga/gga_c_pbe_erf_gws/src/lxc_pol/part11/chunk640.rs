//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 640/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk640(t1371: f64, t1952: f64, t553: f64, t1378: f64, t1971: f64, t5697: f64, t1464: f64, t285: f64, t545: f64, t159: f64, t5984: f64, t169: f64, t274: f64, t301: f64, t922: f64) -> (f64, f64, f64, f64, f64) {
    let t6012 = 0.19753890328909480882e-1_f64 * t1952 * t1371 * t553;
    let t6015 = 0.34679929861433484636e-2_f64 * t5697 * t1378 * t1971;
    let t6028 = 0.40679438125041687114e-2_f64 * t1464 * t545 * t285;
    let t6032 = 0.67153358174671991426e-2_f64 * t5984 * t159 * t285;
    let t6036 = 0.92478548207158653218e0_f64 * t169 * t922 * t274 * t301;
    (t6012, t6015, t6028, t6032, t6036)
}
