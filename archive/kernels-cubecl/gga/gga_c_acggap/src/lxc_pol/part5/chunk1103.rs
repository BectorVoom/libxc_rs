//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1103/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1103<F: Float>(t1907: F, t939: F, t1160: F, t1539: F, t19757: F, t4162: F, t6461: F, t3378: F, t6538: F, t1925: F, t980: F, t4180: F, t6475: F) -> (F, F, F, F, F, F) {
    let t19775 = t939 * t1907;
    let t19780 = t1160 * t19757 * t1539;
    let t19783 = t1160 * t6461 * t4162;
    let t19785 = t3378 * t6538;
    let t19792 = t980 * t1925;
    let t19795 = t4180 * t6475;
    (t19775, t19780, t19783, t19785, t19792, t19795)
}
