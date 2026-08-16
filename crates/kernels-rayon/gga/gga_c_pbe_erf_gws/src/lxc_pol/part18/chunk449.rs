//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 449/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk449(t528: f64, t713: f64, t1365: f64, t153: f64, t274: f64, t542: f64, t745: f64, t164: f64, t762: f64, t1597: f64, t547: f64, t147: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1928 = 0.33245444444444444444e-1_f64 * t528 * t713;
    let t1937 = 0.13287210228946179141e1_f64 * t153 * t1365 * t274;
    let t1939 = t153 * t542 * t745;
    let t1947 = 0.63010814446282235668e-1_f64 * t762 * t164;
    let t1948 = t1597 * t164;
    let t1951 = 0.63010814446282235668e-1_f64 * t528 * t547;
    let t1952 = t837 * t147;
    (t1928, t1937, t1939, t1947, t1948, t1951, t1952)
}
