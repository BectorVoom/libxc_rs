//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1110/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1110<F: Float>(t40582: F, t40592: F, t40598: F, t40606: F, t40612: F, t40615: F, t40619: F, t40623: F, t40626: F, t40628: F, t40634: F, t40638: F, t40647: F, t40651: F, t40654: F, t40662: F) -> (F,) {
    let t41092 = -t40582 - t40592 - t40598 - t40606 - t40612 + t40615 - t40619 + t40623 + t40626 + t40628 + t40634 - t40638 + t40647 - t40651 + t40654 - t40662;
    (t41092,)
}
