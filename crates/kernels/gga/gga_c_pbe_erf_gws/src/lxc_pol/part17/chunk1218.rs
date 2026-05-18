//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1218/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1218<F: Float>(t14817: F, t945: F, t321: F, t47184: F, t50832: F, t14822: F, t4188: F, t6854: F, t14829: F, t1172: F, t1198: F, t318: F) -> (F, F, F, F, F, F, F) {
    let t52799 = t14817 * t945;
    let t52801 = F::new(2.0) * t321 * t52799;
    let t52810 = F::new(6.0) * t50832 * t47184;
    let t52812 = F::new(2.0) * t321 * t14822;
    let t52816 = t4188 * t6854;
    let t52821 = F::new(2.0) * t321 * t14829;
    let t52823 = t1172 * t318 * t1198;
    (t52799, t52801, t52810, t52812, t52816, t52821, t52823)
}
