//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1196/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1196<F: Float>(t101628: F, t101631: F, t101636: F, t101638: F, t101642: F, t101646: F, t101650: F, t101655: F, t101659: F, t101662: F, t101665: F, t101669: F, t100319: F, t5674: F, t93351: F, t1586: F, t25846: F) -> (F, F, F) {
    let t101671 = 2.0 / 27.0 * t101628 - 2.0 / 9.0 * t101631 + t101636 - 4.0 / 27.0 * t101638 - t101642 / 3.0 + t101646 / 18.0 - t101650 + 5.0 / 16.0 * t101655 - t101659 / 9.0 + t101662 + t101665 / 18.0 + 2.0 / 3.0 * t101669;
    let t101676 = t5674 * t93351 * t100319;
    let t101678 = t1586 * t25846;
    (t101671, t101676, t101678)
}
