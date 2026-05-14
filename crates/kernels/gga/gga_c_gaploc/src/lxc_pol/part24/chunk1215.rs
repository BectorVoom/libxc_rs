//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1215/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1215<F: Float>(t10151: F, t10231: F, t1391: F, t1392: F, t1402: F, t1429: F, t2487: F, t30388: F, t34386: F, t34394: F, t34397: F, t34404: F, t34406: F, t34410: F, t34414: F, t34415: F, t34416: F, t34418: F, t34420: F, t34423: F, t34425: F) -> (F,) {
    let t34426 = t30388 - t34386 - 0.92686455430723328401e-1 * t1429 * t1402 * t10231 + 0.11360866949309851756e0 * t2487 * t1391 * t1392 * t10151 - t34394 - t34397 - t34404 - t34406 - t34410 - t34414 - t34415 + t34416 + t34418 - t34420 + t34423 + t34425;
    (t34426,)
}
