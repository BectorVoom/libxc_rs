//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 480/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk480<F: Float>(t1994: F, t2010: F, t2012: F, t2017: F, t1602: F, t163: F, t164: F, t169: F, t171: F, t1947: F, t1948: F, t1951: F, t1955: F, t1958: F, t1962: F, t1966: F, t1969: F, t1973: F, t1977: F, t1980: F) -> (F, F) {
    let t2019 = t1994 + t2010 + t2012 + t2017;
    let t2024 = -t1947 + F::new(0.63010814446282235668e-1) * t1948 + t1951 + t1955 - F::new(0.31505407223141117834e-1) * t1602 * t164 - F::new(0.63010814446282235668e-1) * t1958 - F::new(0.39507780657818961764e-2) * t1962 - t1966 - t1969 - t1973 - t1977 + F::new(0.17961351015381913641e-1) * t1980 - F::new(0.53884053046145740922e-2) * t169 * t171 * t2019 * t163;
    (t2019, t2024)
}
