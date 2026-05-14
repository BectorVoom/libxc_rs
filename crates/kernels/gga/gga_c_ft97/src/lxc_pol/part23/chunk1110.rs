//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1110/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1110<F: Float>(t10491: F, t6318: F, t1491: F, t2999: F, t89: F, t1636: F, t6343: F, t10570: F, t683: F, t2399: F, t6339: F, t1483: F, t3281: F, t38953: F, t6362: F, t24898: F, t56456: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99529 = t10491 * t6318;
    let t99534 = t89 * t2999 * t1491;
    let t99535 = 28.0 / 27.0 * t99534;
    let t99537 = t89 * t1636 * t6343;
    let t99559 = t683 * t10570;
    let t99607 = t89 * t2399 * t6339;
    let t99635 = 28.0 / 81.0 * t3281 * t1483;
    let t99665 = t38953 * t6362;
    let t99672 = t56456 * t24898;
    (t99529, t99534, t99535, t99537, t99559, t99607, t99635, t99665, t99672)
}
