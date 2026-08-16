//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2242/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2242<F: Float>(t28359: F, t838: F, t23069: F, t5572: F, t23062: F, t28383: F, t20986: F, t2628: F, t6605: F, t828: F, t17004: F, t6581: F) -> (F, F, F, F, F) {
    let t98690 = t28359 * t838;
    let t98694 = t23069 * t5572;
    let t98696 = t23062 * t28383;
    let t98701 = t6605 * t2628 * t20986 * t828;
    let t98703 = t6581 * t17004;
    (t98690, t98694, t98696, t98701, t98703)
}
