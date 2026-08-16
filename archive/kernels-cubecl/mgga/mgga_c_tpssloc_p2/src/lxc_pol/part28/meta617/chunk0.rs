//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1934/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1934<F: Float>(t7712: F, t80939: F, t22683: F, t26285: F, t6546: F, t16148: F, t221: F, t26284: F, t16153: F, t26289: F, t6604: F, t80887: F) -> (F, F, F, F, F) {
    let t91167 = t80939 * t7712;
    let t91170 = t6546 * t22683 * t26285;
    let t91173 = t26284 * t221 * t16148;
    let t91176 = t26284 * t221 * t16153;
    let t91179 = t80887 * t6604 * t26289;
    (t91167, t91170, t91173, t91176, t91179)
}
