//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 563/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk563<F: Float>(t7124: F, t871: F, t296: F, t1901: F, t193: F, t446: F, t6272: F, t6298: F, t6359: F, t7033: F, t7038: F, t7042: F, t7047: F, t7051: F, t7055: F, t7059: F, t7093: F, t7098: F, t7102: F, t7107: F, t7111: F, t7116: F, t89: F) -> (F, F, F) {
    let t7125 = t871 * t7124;
    let t7126 = t296 * t7125;
    let t7129 = t6272 + t1901 * t7033 / 9.0 + 2.0 / 3.0 * t446 * t7038 - t446 * t7042 / 3.0 + t446 * t7047 / 3.0 - t446 * t7051 / 3.0 - t6298 - t446 * t7055 / 9.0 - t446 * t7059 / 3.0 + t89 * t193 * t7093 / 3.0 - t446 * t7098 / 3.0 + t6359 + t1901 * t7102 / 9.0 + t446 * t7107 / 3.0 - t446 * t7111 / 3.0 + 2.0 / 3.0 * t446 * t7116 - t446 * t7126 / 3.0;
    (t7125, t7126, t7129)
}
