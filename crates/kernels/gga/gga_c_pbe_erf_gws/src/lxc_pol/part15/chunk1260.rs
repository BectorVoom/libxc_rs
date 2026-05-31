//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1260/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1260<F: Float>(t13820: F, t2409: F, t3207: F, t51153: F, t51156: F, t51162: F, t51168: F, t53487: F, t53493: F, t53498: F, t53503: F, t53509: F, t53510: F, t53513: F, t53516: F, t53517: F, t53520: F, t8589: F) -> F {
    let t53522 = -F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t51153 - t53487 / F::cast_from(16.0_f64) - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t51156 - t53493 / F::cast_from(768.0_f64) + t53498 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51162 + F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t51168 - t53503 - t3207 * t2409 * t8589 * t13820 / F::cast_from(16.0_f64) + t53509 - t53510 / F::cast_from(48.0_f64) + t53513 / F::cast_from(1536.0_f64) + t53516 + t53517 / F::cast_from(24.0_f64) - t53520 / F::cast_from(192.0_f64);
    t53522
}
