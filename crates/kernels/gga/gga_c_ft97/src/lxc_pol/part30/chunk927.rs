//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 927/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk927<F: Float>(t150096: F, t150099: F, t150102: F, t150106: F, t150109: F, t150114: F, t150118: F, t150122: F, t150125: F, t150128: F, t150131: F, t150136: F, t150139: F, t150140: F, t150144: F, t150148: F) -> (F,) {
    let t151247 = -8.0 / 9.0 * t150096 + 4.0 / 9.0 * t150099 - 4.0 / 9.0 * t150102 - t150106 / 3.0 - 2.0 * t150109 - 4.0 / 9.0 * t150114 + 2.0 / 9.0 * t150118 - 2.0 / 9.0 * t150122 - 2.0 / 9.0 * t150125 - 2.0 / 9.0 * t150128 - 8.0 / 9.0 * t150131 - 2.0 / 3.0 * t150136 + t150139 / 3.0 - t150140 / 27.0 + 4.0 * t150144 + t150148 / 9.0;
    (t151247,)
}
