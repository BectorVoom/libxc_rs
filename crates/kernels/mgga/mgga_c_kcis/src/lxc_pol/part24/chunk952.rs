//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 952/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk952<F: Float>(t233: F, t27755: F, t235: F, t5398: F, t2169: F, t7673: F, t8021: F, t283: F, t3226: F) -> (F, F, F, F) {
    let t27756 = t233 * t27755;
    let t27758 = t235 * t5398;
    let t27759 = t2169 * t27758;
    let t27761 = t7673 * t8021;
    let t27763 = t3226 * t283;
    (t27756, t27759, t27761, t27763)
}
