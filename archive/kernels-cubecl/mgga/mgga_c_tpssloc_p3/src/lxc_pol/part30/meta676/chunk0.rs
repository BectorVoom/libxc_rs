//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2107/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2107<F: Float>(t12571: F, t1410: F, t26012: F, t7441: F, t27971: F, t645: F, t72: F, t1437: F, t7445: F, t1863: F, t27975: F, t1864: F, t5445: F) -> (F, F, F, F, F, F) {
    let t96443 = t12571 * t1410;
    let t96454 = t7441 * t26012;
    let t96458 = t72 * t27971 * t645;
    let t96461 = t7445 * t1437;
    let t96462 = t1863 * t96461;
    let t96466 = t72 * t27975 * t645;
    let t96469 = t1864 * t5445;
    (t96443, t96454, t96458, t96462, t96466, t96469)
}
