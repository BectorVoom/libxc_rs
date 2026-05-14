//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 491/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk491<F: Float>(t1286: F, t1310: F, t1337: F, t438: F, t5495: F, t5500: F, t5501: F, t5504: F, t5510: F, t5620: F, t5624: F, t5711: F, t5727: F, t5732: F, t5744: F, t5748: F, t5750: F, t88: F) -> (F,) {
    let t5756 = t5495 * t1310 / 6.0 - t5500 - t5501 * t5504 / 18.0 - t1286 * t5510 / 3.0 + t1286 * t5620 / 6.0 + t1286 * t5624 / 6.0 - t438 * t1337 - t88 * t5748 + 2.0 * t5750 - 2.0 * t5711 - 2.0 * t5727 + 4.0 * t5732 - 2.0 * t5744;
    (t5756,)
}
