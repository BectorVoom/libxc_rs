//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 281/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk281<F: Float>(t3688: F, t3710: F, t2339: F, t2342: F, t2533: F, t3693: F, t3697: F, t3702: F, t3707: F, t3715: F, t3720: F, t3824: F, t3904: F, t3940: F, t241: F, t258: F) -> (F, F) {
    let t3942 = t3688 / 27.0;
    let t3947 = t3710 / 9.0;
    let t3951 = -t3904 / 12.0 + t3940 / 6.0 + t2533 + t2339 + t2342 + t3942 - 2.0 / 27.0 * t3693 + t3697 / 9.0 + 2.0 / 9.0 * t3702 + 2.0 / 9.0 * t3707 + t3947 + t3715 / 9.0 + 2.0 / 3.0 * t3720 - t3824 / 3.0;
    let t3953 = t241 * t3951 * t258;
    (t3951, t3953)
}
