//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 199/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk199<F: Float>(t625: F, t44: F, t49: F, t56: F, t614: F, t617: F, t620: F, t38: F, t45: F) -> (F, F, F, F) {
    let t626 = 8.0 / 3.0 * t625;
    let t627 = -8.0 / 3.0 * t614 * t49 + 5.0 / 6.0 * t44 * t617 - 5.0 / 6.0 * t56 * t620 + t626;
    let t628 = t38 * t627;
    let t631 = t45 * t45;
    (t626, t627, t628, t631)
}
