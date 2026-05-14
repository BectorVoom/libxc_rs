//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1115/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1115<F: Float>(t107998: F, t24543: F, t27764: F, t27772: F, t96925: F, t27777: F, t12001: F, t27476: F, t1882: F, t27484: F, t1434: F, t27879: F, t681: F, t27846: F, t6109: F, t27842: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t107999 = 2.0 / 9.0 * t107998;
    let t108000 = t24543 * t27764;
    let t108001 = 2.0 / 27.0 * t108000;
    let t108002 = t96925 * t27772;
    let t108003 = t108002 / 18.0;
    let t108060 = t96925 * t27777;
    let t108061 = t108060 / 18.0;
    let t108070 = t12001 * t27476;
    let t108072 = t1882 * t27484;
    let t108073 = 2.0 / 9.0 * t108072;
    let t108077 = t1434 * t681 * t27879;
    let t108078 = 2.0 / 3.0 * t108077;
    let t108080 = t6109 * t681 * t27846;
    let t108081 = t108080 / 6.0;
    let t108083 = t1434 * t681 * t27842;
    (t107999, t108000, t108001, t108002, t108003, t108060, t108061, t108070, t108072, t108073, t108077, t108078, t108080, t108081, t108083)
}
