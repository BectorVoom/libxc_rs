//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1138/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1138<F: Float>(t587: F, t589: F, t9278: F, t1407: F, t9548: F, t20887: F, t9305: F, t21417: F, t1397: F, t6603: F, t9287: F, t1415: F, t6699: F, t7030: F) -> (F, F, F, F, F, F, F) {
    let t30606 = t587 * t589 * t9278;
    let t30607 = F::new(0.1022478025437886658e1) * t30606;
    let t30629 = F::new(0.17041300423964777634e0) * t1407 * t9548;
    let t30631 = F::new(0.29792074959875355558e-1) * t9305 * t20887;
    let t30633 = F::new(0.11916829983950142223e0) * t9305 * t21417;
    let t30642 = t1397 * t6603;
    let t30644 = F::new(0.59584149919750711116e-1) * t30642 * t9287;
    let t30647 = F::new(0.59584149919750711116e-1) * t1415 * t6699 * t7030;
    (t30607, t30629, t30631, t30633, t30642, t30644, t30647)
}
