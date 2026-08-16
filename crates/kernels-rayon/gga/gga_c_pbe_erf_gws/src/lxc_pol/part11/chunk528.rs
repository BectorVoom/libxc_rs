//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 528/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk528(t3396: f64, t3462: f64, t3511: f64, t3567: f64, t163: f64, t164: f64, t169: f64, t171: f64, t1947: f64, t1951: f64, t1955: f64, t1966: f64, t1969: f64, t1973: f64, t1977: f64, t2942: f64, t2946: f64, t2950: f64, t2957: f64, t3380: f64) -> (f64, f64) {
    let t3569 = t3396 + t3462 + t3511 + t3567;
    let t3574 = -t1947 + 0.63010814446282235668e-1_f64 * t2942 + t1951 + t1955 - 0.31505407223141117834e-1_f64 * t3380 * t164 - 0.63010814446282235668e-1_f64 * t2946 - 0.39507780657818961764e-2_f64 * t2950 - t1966 - t1969 - t1973 - t1977 + 0.17961351015381913641e-1_f64 * t2957 - 0.53884053046145740922e-2_f64 * t169 * t171 * t3569 * t163;
    (t3569, t3574)
}
