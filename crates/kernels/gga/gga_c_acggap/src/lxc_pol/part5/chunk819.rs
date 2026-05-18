//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 819/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk819<F: Float>(t1922: F, t377: F, t407: F, t6482: F, t1539: F, t6465: F, t1160: F, t6461: F, t1411: F, t1629: F, t1533: F, t1907: F, t394: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6529 = t377 * t1922;
    let t6532 = t6482 * t407;
    let t6535 = t6465 * t1539;
    let t6536 = t1160 * t6535;
    let t6538 = t6461 * t407;
    let t6541 = t1629 * t1411;
    let t6544 = t6465 * t407;
    let t6547 = t6461 * t1533;
    let t6551 = t394 * t1907;
    (t6529, t6532, t6535, t6536, t6538, t6541, t6544, t6547, t6551)
}
