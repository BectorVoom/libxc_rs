//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 830/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk830(t211: f64, t7844: f64, t1798: f64, t2741: f64, t219: f64, t5400: f64, t2584: f64, t5125: f64, t1820: f64, t2666: f64, t5137: f64, t639: f64) -> (f64, f64, f64, f64, f64) {
    let t7845 = t211 * t7844;
    let t7852 = 16.0_f64 / 45.0_f64 * t2741 * t1798;
    let t7853 = t5400 * t219;
    let t7868 = t5125 * t2584;
    let t7870 = 32.0_f64 / 135.0_f64 * t1820 * t7868;
    let t7871 = t5137 * t2666;
    let t7873 = 16.0_f64 / 135.0_f64 * t639 * t7871;
    (t7845, t7852, t7853, t7870, t7873)
}
