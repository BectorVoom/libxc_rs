//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1070/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1070<F: Float>(t1780: F, t5710: F, t1339: F, t8216: F, t6492: F, t8232: F, t1882: F, t26156: F, t38953: F, t6466: F, t26284: F, t26288: F, t26446: F, t8392: F, t26163: F, t26168: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t102682 = t1780 * t5710;
    let t102689 = t8216 * t1339;
    let t102694 = t8232 * t6492;
    let t102697 = 2.0 / 9.0 * t1882 * t26156;
    let t102698 = t38953 * t6466;
    let t102706 = 2.0 / 9.0 * t1882 * t26284;
    let t102708 = 2.0 / 9.0 * t1882 * t26288;
    let t102723 = 2.0 / 27.0 * t8392 * t26446;
    let t102730 = 4.0 / 9.0 * t8392 * t26163;
    let t102732 = 4.0 / 9.0 * t8392 * t26168;
    (t102682, t102689, t102694, t102697, t102698, t102706, t102708, t102723, t102730, t102732)
}
