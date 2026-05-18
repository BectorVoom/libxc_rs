//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1101/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1101<F: Float>(t1038: F, t329: F, t33658: F, t33676: F, t7451: F, t33291: F, t7335: F, t33433: F, t875: F, t7073: F, t29435: F, t829: F, t9895: F) -> (F, F, F, F, F) {
    let t33678 = t1038 * t329;
    let t33680 = t7451 * t33676 * t33678 * t33658;
    let t33682 = t33291 * t7335;
    let t33685 = t33433 * t875;
    let t33687 = t7073 * t33676 * t33678 * t33685;
    let t33690 = t9895 * t829 * t29435;
    (t33680, t33682, t33685, t33687, t33690)
}
