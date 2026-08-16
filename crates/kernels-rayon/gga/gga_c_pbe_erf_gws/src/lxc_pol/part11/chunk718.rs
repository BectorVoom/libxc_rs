//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 718/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk718(t2615: f64, t2643: f64, t3444: f64, t582: f64, t185: f64, t1006: f64, t2756: f64, t2741: f64, t2753: f64, t3563: f64, t616: f64, t3479: f64, t636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10851 = t2615 * t2643;
    let t10871 = t582 * t3444;
    let t10872 = t185 * t10871;
    let t10874 = t1006 * t2756;
    let t10876 = t2741 * t2753;
    let t10878 = t582 * t3563;
    let t10879 = t616 * t10878;
    let t10887 = t3479 * t636;
    (t10851, t10871, t10872, t10874, t10876, t10878, t10879, t10887)
}
