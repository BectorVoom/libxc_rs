//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 971/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk971(t1000: f64, t2784: f64, t1827: f64, t587: f64, t1017: f64, t2768: f64, t7720: f64, t3425: f64, t562: f64, t1821: f64, t1820: f64, t172: f64, t3486: f64) -> (f64, f64, f64, f64) {
    let t10956 = t1000 * t2784;
    let t10957 = t1827 * t10956;
    let t10959 = 8.0_f64 / 45.0_f64 * t587 * t10957;
    let t10960 = t2768 * t1017;
    let t10961 = t7720 * t10960;
    let t10963 = 16.0_f64 / 45.0_f64 * t587 * t10961;
    let t10964 = t3425 * t562;
    let t10965 = t1821 * t10964;
    let t10967 = 16.0_f64 / 45.0_f64 * t1820 * t10965;
    let t10968 = t172 * t3486;
    (t10959, t10963, t10967, t10968)
}
