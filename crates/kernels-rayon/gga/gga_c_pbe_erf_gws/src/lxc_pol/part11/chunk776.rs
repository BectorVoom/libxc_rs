//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 776/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk776(t12623: f64, t639: f64, t1022: f64, t3465: f64, t2677: f64, t1620: f64, t3429: f64, t995: f64, t1821: f64, t1820: f64, t1017: f64, t1827: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12625 = 4.0_f64 / 9.0_f64 * t639 * t12623;
    let t12626 = t3465 * t1022;
    let t12627 = t2677 * t12626;
    let t12629 = 8.0_f64 / 9.0_f64 * t1620 * t12627;
    let t12630 = t3429 * t995;
    let t12631 = t1821 * t12630;
    let t12633 = 8.0_f64 / 15.0_f64 * t1820 * t12631;
    let t12634 = t3429 * t1017;
    let t12635 = t1827 * t12634;
    (t12625, t12626, t12627, t12629, t12630, t12631, t12633, t12634, t12635)
}
