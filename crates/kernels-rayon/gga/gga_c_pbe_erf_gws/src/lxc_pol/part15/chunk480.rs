//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 480/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk480(t1994: f64, t2010: f64, t2012: f64, t2017: f64, t1602: f64, t163: f64, t164: f64, t169: f64, t171: f64, t1947: f64, t1948: f64, t1951: f64, t1955: f64, t1958: f64, t1962: f64, t1966: f64, t1969: f64, t1973: f64, t1977: f64, t1980: f64) -> (f64, f64) {
    let t2019 = t1994 + t2010 + t2012 + t2017;
    let t2024 = -t1947 + 0.63010814446282235668e-1_f64 * t1948 + t1951 + t1955 - 0.31505407223141117834e-1_f64 * t1602 * t164 - 0.63010814446282235668e-1_f64 * t1958 - 0.39507780657818961764e-2_f64 * t1962 - t1966 - t1969 - t1973 - t1977 + 0.17961351015381913641e-1_f64 * t1980 - 0.53884053046145740922e-2_f64 * t169 * t171 * t2019 * t163;
    (t2019, t2024)
}
