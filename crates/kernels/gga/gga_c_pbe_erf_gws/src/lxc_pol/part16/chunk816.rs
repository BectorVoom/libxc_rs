//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 816/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk816<F: Float>(t2160: F, t6542: F, t2289: F, t2293: F, t2262: F, t344: F, t362: F, t2209: F, t825: F, t2182: F, t337: F, t5: F) -> (F, F, F, F, F, F) {
    let t6543 = t6542 * t2160;
    let t6545 = t2289 * t2293;
    let t6552 = F::new(1.0) / t2262 / t344;
    let t6553 = t6552 * t362;
    let t6560 = t825 * t2209;
    let t6562 = t337 * t5 * t2182;
    (t6543, t6545, t6552, t6553, t6560, t6562)
}
