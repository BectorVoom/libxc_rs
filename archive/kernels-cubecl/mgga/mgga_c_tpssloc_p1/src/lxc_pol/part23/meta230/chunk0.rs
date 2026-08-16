//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 880/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk880<F: Float>(t16616: F, t763: F, t2752: F, t5664: F, t4101: F, t4205: F, t5392: F, t634: F, t638: F, t5575: F, t68: F) -> (F, F, F, F, F, F) {
    let t16617 = t16616 * t763;
    let t16625 = t5664 * t2752;
    let t16630 = t4205 * t4101;
    let t16637 = t634 * t5392;
    let t16649 = t638 * t5392;
    let t16673 = t5575 * t68;
    (t16617, t16625, t16630, t16637, t16649, t16673)
}
