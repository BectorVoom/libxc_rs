//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 508/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk508(t435: f64, t619: f64, t2941: f64, t1936: f64, t1423: f64, t522: f64, t1006: f64, t1033: f64, t6: f64, t101: f64, t1459: f64, t1464: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2942 = t435 * t619;
    let t2943 = t2941 * t2942;
    let t2945 = t1936 * t619;
    let t2946 = t2941 * t2945;
    let t2948 = t1423 * t522;
    let t2949 = t1006 * t2948;
    let t2951 = t6 * t1033;
    let t2952 = t2951 * t101;
    let t2953 = t2952 * t1459;
    let t2954 = t520 * t1464;
    (t2942, t2943, t2945, t2946, t2948, t2949, t2951, t2952, t2953, t2954)
}
