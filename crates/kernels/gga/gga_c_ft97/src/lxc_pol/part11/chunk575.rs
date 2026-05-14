//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 575/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk575<F: Float>(t3194: F, t8376: F, t3193: F, t1901: F, t446: F, t8207: F, t8213: F, t8220: F, t8224: F, t8227: F, t8229: F, t8233: F, t8235: F, t8238: F, t8357: F, t8362: F, t8365: F, t8369: F, t8373: F) -> (F, F, F) {
    let t8377 = t3194 * t8376;
    let t8378 = t3193 * t8377;
    let t8381 = -2.0 / 3.0 * t1901 * t8207 + 2.0 / 9.0 * t1901 * t8213 - 2.0 / 3.0 * t1901 * t8220 - t446 * t8224 / 3.0 - 2.0 / 9.0 * t8227 - 2.0 / 3.0 * t8229 - 4.0 / 27.0 * t8233 + 2.0 / 27.0 * t8235 + 4.0 / 9.0 * t446 * t8238 - t446 * t8357 / 3.0 - t446 * t8362 - t446 * t8365 - 2.0 / 3.0 * t1901 * t8369 + 2.0 / 3.0 * t1901 * t8373 - 2.0 / 9.0 * t1901 * t8378;
    (t8377, t8378, t8381)
}
