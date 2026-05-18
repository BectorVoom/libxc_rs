//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 521/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk521<F: Float>(t1821: F, t2555: F, t587: F, t1661: F, t197: F, t1663: F, t950: F, t418: F) -> (F, F, F, F, F) {
    let t2556 = t1821 * t2555;
    let t2558 = F::new(8.0) / F::new(45.0) * t587 * t2556;
    let t2559 = t1661 * t197;
    let t2560 = t1663 * t950;
    let t2561 = t2560 * t418;
    (t2556, t2558, t2559, t2560, t2561)
}
