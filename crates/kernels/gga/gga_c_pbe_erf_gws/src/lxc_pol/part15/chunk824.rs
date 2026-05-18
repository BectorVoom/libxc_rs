//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 824/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk824<F: Float>(t2332: F, t899: F, t900: F, t907: F, t329: F, t6594: F, t378: F, t2271: F, t2365: F, t822: F, t833: F, t2367: F, t2397: F) -> (F, F, F, F, F, F, F, F) {
    let t6717 = t899 * t900 * t2332;
    let t6718 = t6717 * t907;
    let t6729 = t329 * t6594;
    let t6731 = F::new(455.0) / F::new(1296.0) * t6729 * t378;
    let t6744 = t2271 * t2365;
    let t6745 = t822 * t6744;
    let t6746 = t6745 * t833;
    let t6748 = t2367 * t2397;
    (t6717, t6718, t6729, t6731, t6744, t6745, t6746, t6748)
}
