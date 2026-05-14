//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1109/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1109<F: Float>(t1114: F, t50935: F, t13793: F, t1112: F, t2306: F, t3074: F, t833: F, t837: F, t14657: F, t51721: F, t13984: F, t13972: F, t14799: F, t22509: F, t4166: F, t1176: F, t21518: F, t367: F) -> (F, F, F, F, F, F, F) {
    let t53571 = t1114 * t50935;
    let t53572 = t53571 * t13793;
    let t53577 = t3074 * t2306 * t1112 * t837 * t833;
    let t53578 = 7.0 / 144.0 * t53577;
    let t53579 = t14657 * t51721;
    let t53581 = t53571 * t13984;
    let t53583 = t13972 * t14799;
    let t53584 = 7.0 / 1152.0 * t53583;
    let t53585 = t22509 * t4166;
    let t53592 = t1176 * t367 * t21518;
    (t53572, t53578, t53579, t53581, t53584, t53585, t53592)
}
