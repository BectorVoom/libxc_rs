//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 529/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk529<F: Float>(t3457: F, t406: F, t1629: F, t3073: F, t1647: F, t864: F, t1035: F, t124: F, t56: F, t2029: F, t142: F, t174: F) -> (F, F, F, F, F, F) {
    let t4241 = t3457 * t406;
    let t4242 = t1629 * t4241;
    let t4244 = 0.26341796731742046394e1 * t3073 * t4242;
    let t4245 = t1647 * t864;
    let t4246 = t1035 * t4245;
    let t4254 = t124 * t56;
    let t4255 = t4254 * t2029;
    let t4256 = t142 * t174;
    (t4241, t4244, t4246, t4254, t4255, t4256)
}
