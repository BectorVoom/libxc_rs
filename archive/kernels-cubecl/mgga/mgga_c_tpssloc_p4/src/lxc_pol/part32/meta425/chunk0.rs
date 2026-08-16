//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1636/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1636<F: Float>(t1437: F, t4021: F, t5445: F, t645: F, t1409: F, t65: F, t67: F, t1864: F, t3966: F, t5392: F, t628: F, t17635: F) -> (F, F, F, F, F, F) {
    let t19313 = t1437 * t4021;
    let t19318 = t5445 * t645;
    let t19322 = t1409 * t65 * t67;
    let t19323 = t1864 * t3966;
    let t19326 = t5392 * t628;
    let t19331 = t17635 * t65;
    (t19313, t19318, t19322, t19323, t19326, t19331)
}
