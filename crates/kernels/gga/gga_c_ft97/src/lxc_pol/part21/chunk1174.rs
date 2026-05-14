//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1174/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1174<F: Float>(t116082: F, t27: F, t370: F, t89: F, t1882: F, t29622: F, t102226: F, t102228: F, t102230: F, t102231: F, t116708: F, t116711: F, t116713: F, t116716: F, t116720: F, t116724: F) -> (F, F, F) {
    let t116728 = t89 * t27 * t370 * t116082;
    let t116729 = t1882 * t29622;
    let t116730 = t116729 / 9.0;
    let t116731 = 2.0 * t116708 + t116711 - t102226 - t116713 + t116716 / 6.0 + t116720 / 9.0 + t116724 / 3.0 - t102228 - t102230 - t102231 - t116728 - t116730;
    (t116728, t116729, t116731)
}
