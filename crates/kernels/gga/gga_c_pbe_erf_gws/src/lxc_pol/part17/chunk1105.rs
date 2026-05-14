//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1105/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1105<F: Float>(t13793: F, t53229: F, t13792: F, t8790: F, t13776: F, t37214: F, t3975: F, t13820: F, t2409: F, t3207: F, t51153: F, t51156: F, t51162: F, t51168: F, t53487: F, t53493: F, t53498: F, t53503: F, t53509: F, t53510: F, t53513: F, t8589: F) -> (F,) {
    let t53515 = t53229 * t13793;
    let t53516 = 7.0 / 72.0 * t53515;
    let t53517 = t13792 * t8790;
    let t53520 = t13776 * t3975 * t37214;
    let t53522 = -7.0 / 1152.0 * t51153 - t53487 / 16.0 - 35.0 / 216.0 * t51156 - t53493 / 768.0 + t53498 / 768.0 - 7.0 / 144.0 * t51162 + 35.0 / 108.0 * t51168 - t53503 - t3207 * t2409 * t8589 * t13820 / 16.0 + t53509 - t53510 / 48.0 + t53513 / 1536.0 + t53516 + t53517 / 24.0 - t53520 / 192.0;
    (t53522,)
}
