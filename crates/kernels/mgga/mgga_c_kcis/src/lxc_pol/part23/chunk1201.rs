//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1201/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1201<F: Float>(t17311: F, t27506: F, t12338: F, t28573: F, t2253: F, t52933: F, t2069: F, t27553: F, t4189: F, t5900: F, t94816: F, t39296: F, t8186: F) -> (F, F, F, F, F, F) {
    let t97635 = F::new(4.0) * t17311 * t27506;
    let t97637 = F::new(4.0) * t12338 * t28573;
    let t97638 = t52933 * t2253;
    let t97641 = F::new(2.0) * t4189 * t27553 * t2069;
    let t97643 = F::new(4.0) * t94816 * t5900;
    let t97645 = F::new(2.0) * t39296 * t8186;
    (t97635, t97637, t97638, t97641, t97643, t97645)
}
