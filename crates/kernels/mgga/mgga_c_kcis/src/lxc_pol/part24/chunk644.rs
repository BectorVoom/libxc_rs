//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 644/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk644<F: Float>(t1267: F, t7773: F, t5329: F, t20: F, t251: F, t982: F, t1240: F, t209: F, t24: F, t2196: F) -> (F, F, F, F, F) {
    let t7774 = t7773 * t1267;
    let t7775 = t5329 * t7774;
    let t7779 = t251 * t982 * t20;
    let t7780 = t1240 * t7779;
    let t7783 = t209 * t24;
    let t7784 = t7783 * t2196;
    (t7774, t7775, t7779, t7780, t7784)
}
