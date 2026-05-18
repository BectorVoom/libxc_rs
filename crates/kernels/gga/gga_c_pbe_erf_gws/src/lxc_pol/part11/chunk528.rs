//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 528/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk528<F: Float>(t3396: F, t3462: F, t3511: F, t3567: F, t163: F, t164: F, t169: F, t171: F, t1947: F, t1951: F, t1955: F, t1966: F, t1969: F, t1973: F, t1977: F, t2942: F, t2946: F, t2950: F, t2957: F, t3380: F) -> (F, F) {
    let t3569 = t3396 + t3462 + t3511 + t3567;
    let t3574 = -t1947 + F::new(0.63010814446282235668e-1) * t2942 + t1951 + t1955 - F::new(0.31505407223141117834e-1) * t3380 * t164 - F::new(0.63010814446282235668e-1) * t2946 - F::new(0.39507780657818961764e-2) * t2950 - t1966 - t1969 - t1973 - t1977 + F::new(0.17961351015381913641e-1) * t2957 - F::new(0.53884053046145740922e-2) * t169 * t171 * t3569 * t163;
    (t3569, t3574)
}
