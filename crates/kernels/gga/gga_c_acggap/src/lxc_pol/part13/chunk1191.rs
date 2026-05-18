//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1191/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1191<F: Float>(t2268: F, t30792: F, t7493: F, t7642: F, t8480: F, t30216: F, t8665: F, t34406: F, t4465: F, t30154: F, t36209: F, t7586: F) -> (F, F, F, F, F) {
    let t36289 = t30792 * t2268;
    let t36292 = t7493 * t8480 * t7642;
    let t36293 = F::new(0.10718504529517434243e-2) * t36292;
    let t36294 = t30216 * t8665;
    let t36296 = t34406 * t4465;
    let t36299 = t30154 * t7586 * t36209;
    (t36289, t36293, t36294, t36296, t36299)
}
