//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 526/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk526<F: Float>(t1022: F, t649: F, t661: F, t1621: F, t1620: F, t1032: F, t586: F) -> (F, F, F, F, F) {
    let t2607 = t649 * t1022;
    let t2608 = t2607 * t661;
    let t2609 = t1621 * t2608;
    let t2611 = F::new(4.0) / F::new(15.0) * t1620 * t2609;
    let t2612 = t1032 * t586;
    (t2607, t2608, t2609, t2611, t2612)
}
