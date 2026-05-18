//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1247/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1247<F: Float>(t14657: F, t51721: F, t13984: F, t53571: F, t13972: F, t14799: F, t1176: F, t21518: F, t367: F, t3974: F, t3990: F, t8939: F) -> (F, F, F, F) {
    let t53579 = t14657 * t51721;
    let t53581 = t53571 * t13984;
    let t53583 = t13972 * t14799;
    let t53592 = t1176 * t367 * t21518;
    let t53595 = t53592 * t3990 * t3974 * t8939;
    (t53579, t53581, t53583, t53595)
}
