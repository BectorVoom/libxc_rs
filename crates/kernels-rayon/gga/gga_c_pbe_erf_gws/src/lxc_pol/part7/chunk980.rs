//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 980/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk980(t1500: f64, t5656: f64, t142: f64, t525: f64, t5645: f64, t5602: f64, t2031: f64, t5842: f64, t1597: f64, t1917: f64, t528: f64, t5420: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18133 = t1500 * t5656;
    let t18137 = t525 * t142 * t5645;
    let t18140 = t1500 * t5602;
    let t18144 = t2031 * t142 * t5842;
    let t18146 = t1597 * t1917;
    let t18149 = 0.19947266666666666666e0_f64 * t528 * t5420;
    (t18133, t18137, t18140, t18144, t18146, t18149)
}
