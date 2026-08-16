//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 902/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk902(t1044: f64, t1642: f64, t1413: f64, t5522: f64, t639: f64, t2584: f64, t5125: f64, t1820: f64, t2666: f64, t5137: f64, t2673: f64, t4934: f64) -> (f64, f64, f64, f64) {
    let t7863 = t1044 * t1642;
    let t7864 = t7863 * t1413;
    let t7865 = t5522 * t7864;
    let t7867 = 4.0_f64 / 27.0_f64 * t639 * t7865;
    let t7868 = t5125 * t2584;
    let t7870 = 32.0_f64 / 135.0_f64 * t1820 * t7868;
    let t7871 = t5137 * t2666;
    let t7873 = 16.0_f64 / 135.0_f64 * t639 * t7871;
    let t7874 = t4934 * t2673;
    (t7867, t7870, t7873, t7874)
}
