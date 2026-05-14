//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1344/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1344<F: Float>(t105843: F, t105847: F, t105849: F, t105853: F, t105857: F, t105860: F, t105863: F, t105866: F, t105871: F, t105876: F, t95301: F, t96120: F, t105329: F, t1369: F, t28: F, t586: F) -> (F, F) {
    let t105878 = 2.0 * t105843 + t105847 + t105849 + 8.0 / 27.0 * t95301 - 6.0 * t105853 - t105857 + t105860 / 3.0 + t105863 + 24.0 * t105866 - 3.0 / 4.0 * t105871 - 3.0 / 8.0 * t105876 + t96120;
    let t105882 = t1369 * t28 * t586 * t105329;
    (t105878, t105882)
}
