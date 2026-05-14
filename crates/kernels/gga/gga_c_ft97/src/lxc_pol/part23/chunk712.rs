//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 712/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk712<F: Float>(t18506: F, t9791: F, t2606: F, t11593: F, t14114: F, t18455: F, t18457: F, t18461: F, t18464: F, t18468: F, t18473: F, t18476: F, t18479: F, t18483: F, t18488: F, t18493: F, t18499: F, t18503: F, t1901: F, t446: F) -> (F, F) {
    let t18507 = t9791 * t18506;
    let t18508 = t2606 * t18507;
    let t18511 = -2.0 / 27.0 * t18455 - 2.0 / 27.0 * t18457 + 2.0 / 27.0 * t1901 * t18461 + 4.0 / 9.0 * t1901 * t18464 - 4.0 / 27.0 * t1901 * t18468 + t1901 * t18473 / 9.0 + 2.0 / 9.0 * t1901 * t18476 + 2.0 / 9.0 * t1901 * t18479 + 2.0 / 9.0 * t446 * t18483 + t446 * t18488 / 3.0 + 2.0 / 3.0 * t446 * t18493 + 8.0 / 27.0 * t14114 - 8.0 / 9.0 * t11593 * t18499 - 2.0 / 9.0 * t1901 * t18503 - 2.0 / 9.0 * t1901 * t18508;
    (t18507, t18511)
}
