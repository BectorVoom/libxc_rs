//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2098/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2098<F: Float>(t16816: F, t16839: F, t4180: F, t4182: F, t5593: F, t9638: F, t5527: F, t776: F, t820: F, t9607: F, t16753: F, t819: F) -> (F, F, F, F, F) {
    let t16841 = t4180 * t16839 * t16816;
    let t16845 = t4180 * t16839 * t4182;
    let t16848 = t9638 * t5593;
    let t16851 = t5527 * t776;
    let t16853 = t9607 * t820 * t16851;
    let t16859 = t819 * t820 * t16753;
    (t16841, t16845, t16848, t16853, t16859)
}
