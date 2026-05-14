//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1136/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1136<F: Float>(t108060: F, t108072: F, t108035: F, t108039: F, t108043: F, t108047: F, t108051: F, t108055: F, t108059: F, t108063: F, t108068: F, t108070: F, t108077: F, t108080: F, t108083: F, t108114: F) -> (F, F, F, F, F) {
    let t110060 = t108060 / 54.0;
    let t110064 = 2.0 / 27.0 * t108072;
    let t110065 = 2.0 / 27.0 * t108035 - t108039 / 18.0 + t108043 / 18.0 + 2.0 / 9.0 * t108047 - 2.0 / 9.0 * t108051 - t108055 / 18.0 + t108059 / 3.0 + t110060 - 4.0 / 9.0 * t108063 + t108068 / 12.0 + 22.0 / 27.0 * t108070 - t110064;
    let t110067 = 2.0 / 9.0 * t108077;
    let t110068 = t108080 / 18.0;
    let t110069 = 2.0 / 9.0 * t108083;
    let t110077 = 2.0 / 27.0 * t108114;
    (t110065, t110067, t110068, t110069, t110077)
}
