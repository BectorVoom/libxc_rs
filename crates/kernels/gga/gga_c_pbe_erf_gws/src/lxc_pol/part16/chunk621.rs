//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 621/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk621<F: Float>(t1049: F, t679: F, t1988: F, t1992: F, t1997: F, t2002: F, t2006: F, t231: F, t2534: F, t2535: F, t2558: F, t2564: F, t2569: F, t2574: F, t2578: F, t2583: F, t2587: F, t2960: F, t2962: F) -> F {
    let t2965 = t1049 * t679;
    let t2968 = t1988 + F::new(4.0) / F::new(3.0) * t1992 + F::new(4.0) / F::new(3.0) * t2960 + t2534 + t2535 - t2558 + t2564 - t2569 + t2574 + t2578 + t2583 + t2587 + F::new(4.0) / F::new(3.0) * t2962 * t231 + F::new(4.0) / F::new(3.0) * t2965 + F::cast_from(0.10821041362364843377e0_f64) * t1997 + t2002 + t2006;
    t2968
}
