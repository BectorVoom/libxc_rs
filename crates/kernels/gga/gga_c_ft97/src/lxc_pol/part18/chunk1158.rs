//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1158/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1158<F: Float>(t100411: F, t10974: F, t1901: F, t100372: F, t100376: F, t100380: F, t100384: F, t100388: F, t100392: F, t100395: F, t100398: F, t100402: F, t100406: F, t100410: F, t1307: F, t8216: F) -> (F, F, F) {
    let t100413 = t1901 * t100411 * t10974;
    let t100415 = -2.0 / 9.0 * t100372 + 5.0 / 81.0 * t100376 + t100380 / 3.0 - 4.0 / 9.0 * t100384 - 4.0 / 9.0 * t100388 + 4.0 / 27.0 * t100392 + 2.0 / 3.0 * t100395 + 2.0 / 3.0 * t100398 + 4.0 / 9.0 * t100402 + t100406 / 9.0 - t100410 - 8.0 / 27.0 * t100413;
    let t100417 = t8216 * t1307;
    (t100413, t100415, t100417)
}
