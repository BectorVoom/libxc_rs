//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1475/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1475<F: Float>(t18188: F, t19288: F, t12560: F, t12561: F, t12562: F, t12563: F, t12564: F, t12565: F, t9225: F, t5385: F, t604: F, t5389: F, t645: F) -> (F, F, F, F) {
    let t19289 = t18188 + t19288;
    let t19297 = t12560 + t12561 + t12562 + t12563 + t12564 - t12565 - t9225;
    let t19299 = t5385 * t604;
    let t19310 = t5389 * t645;
    (t19289, t19297, t19299, t19310)
}
