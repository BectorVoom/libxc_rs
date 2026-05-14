//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1096/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1096<F: Float>(t13917: F, t13919: F, t9603: F, t13792: F, t8602: F, t14767: F, t2379: F, t13791: F, t3039: F, t13984: F, t14657: F, t51714: F, t13793: F, t51584: F, t1144: F, t4387: F, t859: F) -> (F, F, F, F, F, F, F, F) {
    let t53671 = t13917 * t13919 * t9603;
    let t53675 = t13792 * t8602;
    let t53677 = t14767 * t2379;
    let t53688 = t3039 * t13791;
    let t53689 = t53688 * t13984;
    let t53691 = t14657 * t51714;
    let t53693 = t53688 * t13793;
    let t53695 = t14657 * t51584;
    let t53699 = t859 * t1144 * t4387;
    (t53671, t53675, t53677, t53689, t53691, t53693, t53695, t53699)
}
