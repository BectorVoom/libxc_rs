//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 789/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk789<F: Float>(t2062: F, t8519: F, t1167: F, t6854: F, t1161: F, t6781: F, t829: F, t830: F, t1143: F, t898: F) -> (F, F, F, F, F) {
    let t8520 = t8519 * t2062;
    let t8546 = t1167 * t6854;
    let t8582 = t6781 * t1161;
    let t8584 = t829 * t830 * t8582;
    let t8589 = t1143 * t898;
    (t8520, t8546, t8582, t8584, t8589)
}
