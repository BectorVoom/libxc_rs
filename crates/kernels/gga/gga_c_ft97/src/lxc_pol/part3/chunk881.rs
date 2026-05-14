//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 881/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk881<F: Float>(t19278: F, t19298: F, t19301: F, t19304: F, t10797: F, t19273: F, t19276: F, t19283: F, t19287: F, t19292: F, t19295: F, t19826: F, t19836: F, t19849: F, t871: F, t296: F) -> (F, F) {
    let t19852 = 2.0 / 9.0 * t19278;
    let t19857 = t19298 / 9.0;
    let t19858 = 2.0 / 9.0 * t19301;
    let t19859 = 2.0 / 27.0 * t19304;
    let t19860 = 2.0 / 9.0 * t19273 + 4.0 / 3.0 * t19276 - t19852 - t10797 + 2.0 * t19283 - t19287 / 3.0 - 6.0 * t19292 + 4.0 * t19295 + t19857 - t19858 + t19859;
    let t19862 = t19826 + t19836 + t19849 + t19860;
    let t19863 = t871 * t19862;
    let t19864 = t296 * t19863;
    (t19863, t19864)
}
