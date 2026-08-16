//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2214/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2214<F: Float>(t1888: F, t25045: F, t86873: F, t214: F, t5631: F, t1880: F, t6572: F, t22986: F, t23270: F, t5657: F, t776: F, t857: F) -> (F, F, F, F) {
    let t98125 = t1888 * t86873 * t25045;
    let t98133 = t214 * t5631;
    let t98135 = t1880 * t98133 * t6572;
    let t98148 = t22986 * t23270 * t857 * t5657 * t776;
    (t98125, t98133, t98135, t98148)
}
