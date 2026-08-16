//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 852/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk852<F: Float>(t1628: F, t2833: F, t2815: F, t1589: F, t2792: F, t447: F, t7892: F, t6964: F, t1: F, t7887: F) -> (F, F, F, F, F, F) {
    let t8393 = t1628 * t2833;
    let t8398 = t1628 * t2815;
    let t8403 = t1589 * t2792;
    let t8406 = t7892 * t447;
    let t8407 = t6964 * t8406;
    let t8410 = t7887 * t1;
    (t8393, t8398, t8403, t8406, t8407, t8410)
}
