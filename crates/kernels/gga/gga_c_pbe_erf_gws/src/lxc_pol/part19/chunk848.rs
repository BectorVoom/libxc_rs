//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 848/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk848<F: Float>(t1049: F, t1986: F, t2007: F, t2970: F, t1: F, t2522: F, t3: F, t672: F, t2000: F, t20: F, t2653: F, t2004: F) -> (F, F, F, F, F) {
    let t8405 = t1049 * t1986;
    let t8408 = t2970 * t2007;
    let t8411 = t2522 * t1 * t3;
    let t8413 = F::new(0.21642082724729686754e0) * t8411 * t672;
    let t8414 = t2970 * t2000;
    let t8424 = t2653 * t20;
    let t8425 = t8424 * t2004;
    (t8405, t8408, t8413, t8414, t8425)
}
