//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 710/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk710<F: Float>(t14616: F, t296: F, t2749: F, t4176: F, t840: F, t4299: F, t824: F, t871: F, t10478: F, t319: F) -> (F, F, F, F, F) {
    let t15277 = t296 * t14616;
    let t15281 = t840 * t2749 * t4176;
    let t15284 = t4299 * t824;
    let t15286 = t840 * t871 * t15284;
    let t15290 = t10478 * t319;
    (t15277, t15281, t15284, t15286, t15290)
}
