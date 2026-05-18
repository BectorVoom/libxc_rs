//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1242/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1242<F: Float>(t13888: F, t3306: F, t353: F, t859: F, t14404: F, t19906: F, t13917: F, t3258: F, t51021: F, t51023: F, t1114: F, t50942: F) -> (F, F, F, F) {
    let t53220 = t859 * t353 * t13888 * t3306;
    let t53224 = F::new(7.0) / F::new(72.0) * t19906 * t14404;
    let t53227 = t13917 * t51021 * t3258 * t51023;
    let t53229 = t1114 * t50942;
    (t53220, t53224, t53227, t53229)
}
