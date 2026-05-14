//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 690/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk690<F: Float>(t432: F, t4436: F, t110: F, t8411: F, t492: F, t1871: F, t488: F, t3103: F, t942: F, t376: F, t4547: F, t89: F, t4495: F, t452: F, t499: F, t15885: F) -> (F, F, F, F, F, F) {
    let t16110 = t4436 * t432;
    let t16112 = t8411 * t110 * t16110;
    let t16115 = t4436 * t492;
    let t16117 = t1871 * t488 * t16115;
    let t16120 = t942 * t3103;
    let t16122 = t1871 * t110 * t16120;
    let t16126 = t89 * t376 * t4547;
    let t16129 = t452 * t499 * t4495;
    let t16133 = t452 * t110 * t15885;
    (t16112, t16117, t16122, t16126, t16129, t16133)
}
