//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 855/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk855<F: Float>(t209: F, t44303: F, t44351: F, t44412: F, t44452: F, t44501: F, t44545: F, t44615: F, t44666: F, t37275: F, t921: F, t2497: F, t3553: F, t4349: F) -> (F, F, F) {
    let t44670 = (t44303 + t44351 + t44412 + t44452 + t44501 + t44545 + t44615 + t44666) * t209;
    let t44671 = t37275 * t921;
    let t44674 = F::new(6.0) * t4349 * t3553 * t2497;
    (t44670, t44671, t44674)
}
