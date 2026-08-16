//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1086/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1086(t13953: f64, t3966: f64, t1178: f64, t2231: f64, t371: f64, t3983: f64, t3979: f64, t3997: f64, t2412: f64, t3959: f64, t2409: f64, t6787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13954 = t13953 * t3966;
    let t13955 = 7.0_f64 / 144.0_f64 * t13954;
    let t13957 = t371 * t1178 * t2231;
    let t13958 = t3983 * t13957;
    let t13964 = t3979 * t3997;
    let t13965 = 7.0_f64 / 2304.0_f64 * t13964;
    let t13966 = t3959 * t2412;
    let t13968 = t2409 * t6787;
    (t13954, t13955, t13957, t13958, t13964, t13965, t13966, t13968)
}
