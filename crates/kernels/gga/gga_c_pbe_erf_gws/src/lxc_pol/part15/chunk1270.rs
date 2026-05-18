//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1270/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1270<F: Float>(t1118: F, t13859: F, t14682: F, t2158: F, t51530: F, t13917: F, t13919: F, t9347: F, t9603: F, t13792: F, t8602: F, t14767: F, t2379: F) -> (F, F, F, F, F, F) {
    let t53664 = t13859 * t14682 * t1118 * t2158;
    let t53666 = F::new(119.0) / F::new(1728.0) * t51530;
    let t53668 = t13917 * t13919 * t9347;
    let t53671 = t13917 * t13919 * t9603;
    let t53675 = t13792 * t8602;
    let t53677 = t14767 * t2379;
    (t53664, t53666, t53668, t53671, t53675, t53677)
}
