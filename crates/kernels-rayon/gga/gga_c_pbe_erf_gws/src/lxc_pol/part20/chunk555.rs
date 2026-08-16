//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 555/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk555(t1563: f64, t967: f64, t2873: f64, t506: f64, t10: f64, t127: f64, t1511: f64, t1519: f64, t1540: f64, t1542: f64, t1555: f64, t1558: f64, t1561: f64, t2862: f64, t2865: f64, t2868: f64, t2876: f64, t2879: f64, t2881: f64, t2886: f64, t2891: f64, t481: f64, t496: f64) -> (f64, f64, f64) {
    let t2893 = t1563 * t967;
    let t2897 = t506 * t2873;
    let t2900 = -t1511 + t2862 + t1519 + t2865 + t2868 - t2876 + t1540 + t1542 / 6.0_f64 + t2879 / 6.0_f64 + 3.0_f64 / 2.0_f64 * t496 * t10 * t2881 - t496 * t2886 / 2.0_f64 + t1555 + 0.73452e0_f64 * t1558 + t1561 + 0.73452e0_f64 * t2891 + 0.587616e1_f64 * t127 * t2893 * t481 - 0.146904e1_f64 * t127 * t2897;
    (t2893, t2897, t2900)
}
