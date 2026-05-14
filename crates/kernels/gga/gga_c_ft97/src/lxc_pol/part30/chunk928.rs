//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 928/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk928<F: Float>(t140784: F, t140795: F, t140797: F, t141577: F, t150154: F, t150158: F, t150162: F, t150165: F, t150168: F, t150171: F, t150175: F, t150179: F, t150184: F, t150188: F, t150194: F, t150199: F) -> (F,) {
    let t151264 = -2.0 * t150154 - 4.0 * t150158 - 2.0 / 9.0 * t150162 + t150165 / 18.0 + 4.0 / 9.0 * t150168 - t150171 / 9.0 - t150175 / 36.0 - t150179 / 36.0 - 20.0 / 3.0 * t150184 + 8.0 / 3.0 * t150188 - t141577 + t140784 / 18.0 + 2.0 / 27.0 * t140795 - t140797 / 27.0 + 4.0 / 9.0 * t150194 + t150199 / 12.0;
    (t151264,)
}
