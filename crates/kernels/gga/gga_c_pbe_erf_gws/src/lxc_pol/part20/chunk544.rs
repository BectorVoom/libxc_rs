//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 544/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk544<F: Float>(t2968: F, t2973: F, t2975: F, t2977: F, t163: F, t164: F, t169: F, t171: F, t1947: F, t1948: F, t1951: F, t1955: F, t1958: F, t1962: F, t1966: F, t1969: F, t1973: F, t1977: F, t1980: F, t2523: F, t2942: F, t2946: F, t2950: F, t2957: F) -> (F, F) {
    let t2979 = t2968 + t2973 + t2975 + t2977;
    let t2984 = -t1947 + 0.31505407223141117834e-1 * t1948 + t1951 + t1955 + 0.31505407223141117834e-1 * t2942 - 0.31505407223141117834e-1 * t2523 * t164 - 0.31505407223141117834e-1 * t2946 - 0.19753890328909480882e-2 * t2950 - 0.31505407223141117834e-1 * t1958 - t1966 - t1969 - 0.19753890328909480882e-2 * t1962 - t1973 - t1977 + 0.89806755076909568204e-2 * t1980 + 0.89806755076909568204e-2 * t2957 - 0.53884053046145740922e-2 * t169 * t171 * t2979 * t163;
    (t2979, t2984)
}
