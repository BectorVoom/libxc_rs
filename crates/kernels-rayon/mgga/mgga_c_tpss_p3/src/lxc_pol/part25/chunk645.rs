//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 645/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk645(t345: f64, t3949: f64, t947: f64, t242: f64, t3932: f64, t949: f64, t3931: f64, t1407: f64, t2741: f64, t2751: f64, t967: f64, t2459: f64, t2761: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3950 = t3949 * t345;
    let t3951 = t947 * t3950;
    let t3952 = t242 * t3951;
    let t3955 = t3932 * t949;
    let t3956 = t3931 * t3955;
    let t3962 = t1407 * t949;
    let t3963 = t2741 * t3962;
    let t3968 = t2751 * t1407;
    let t3969 = t242 * t3968;
    let t3970 = t967 * t3969;
    let t3972 = t2761 * t2459;
    (t3950, t3952, t3955, t3956, t3962, t3963, t3969, t3970, t3972)
}
