//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 529/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk529<F: Float>(t529: F, t1555: F, t547: F, t524: F, t1596: F, t544: F, t3729: F, t41: F, t1287: F, t1558: F, t382: F, t4144: F, t4148: F, t525: F, t526: F) -> (F, F, F, F, F, F, F, F) {
    let t530 = t529 < -F::new(0.66725e-1);
    let t4346 = F::new(1.0) / t1555 / t547;
    let t4347 = t524 * t4346;
    let t4348 = t1596 * t1596;
    let t4349 = t544 * t544;
    let t4350 = F::new(1.0) / t4349;
    let t4351 = t4348 * t4350;
    let t4354 = t3729 * t41;
    let t4368 = piecewise3::<F>(t530, F::new(0.0), F::new(10.0) / F::new(9.0) * t525 * t4354 * t382 - F::new(20.0) / F::new(27.0) * t525 * t1558 * t1287 + F::new(40.0) / F::new(81.0) * t525 * t526 * t4144 - F::new(10.0) / F::new(27.0) * t525 * t526 * t4148);
    (t4346, t4347, t4348, t4349, t4350, t4351, t4354, t4368)
}
