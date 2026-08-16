//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 649/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk649<F: Float>(t10332: F, t10378: F, t10413: F, t10460: F, t10504: F, t10560: F, t10607: F, t10621: F, t1022: F, t935: F) -> (F, F) {
    let t10624 = t10332 + t10378 + t10413 + t10460 + t10504 + t10560 + t10607 + t10621;
    let t10627 = t1022 * t935;
    (t10624, t10627)
}
