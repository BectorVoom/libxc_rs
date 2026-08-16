//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 625/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk625(t2968: f64, t2973: f64, t2975: f64, t2977: f64, t163: f64, t164: f64, t169: f64, t171: f64, t1947: f64, t1948: f64, t1951: f64, t1955: f64, t1958: f64, t1962: f64, t1966: f64, t1969: f64, t1973: f64, t1977: f64, t1980: f64, t2523: f64, t2942: f64, t2946: f64, t2950: f64, t2957: f64) -> (f64, f64) {
    let t2979 = t2968 + t2973 + t2975 + t2977;
    let t2984 = -t1947 + 0.31505407223141117834e-1_f64 * t1948 + t1951 + t1955 + 0.31505407223141117834e-1_f64 * t2942 - 0.31505407223141117834e-1_f64 * t2523 * t164 - 0.31505407223141117834e-1_f64 * t2946 - 0.19753890328909480882e-2_f64 * t2950 - 0.31505407223141117834e-1_f64 * t1958 - t1966 - t1969 - 0.19753890328909480882e-2_f64 * t1962 - t1973 - t1977 + 0.89806755076909568204e-2_f64 * t1980 + 0.89806755076909568204e-2_f64 * t2957 - 0.53884053046145740922e-2_f64 * t169 * t171 * t2979 * t163;
    (t2979, t2984)
}
