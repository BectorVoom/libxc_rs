//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 553/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk553<F: Float>(t1636: F, t377: F, t553: F, t848: F, t1603: F, t394: F, t3457: F, t406: F, t1629: F, t3073: F, t1647: F, t864: F) -> (F, F, F, F, F, F) {
    let t4234 = F::new(0.13170898365871023197e1) * t377 * t1636;
    let t4235 = t848 * t553;
    let t4237 = t394 * t1603;
    let t4241 = t3457 * t406;
    let t4242 = t1629 * t4241;
    let t4244 = F::new(0.26341796731742046394e1) * t3073 * t4242;
    let t4245 = t1647 * t864;
    (t4234, t4235, t4237, t4241, t4244, t4245)
}
