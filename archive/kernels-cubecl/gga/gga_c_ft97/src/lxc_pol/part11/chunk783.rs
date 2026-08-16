//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 783/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk783<F: Float>(t10249: F, t10603: F, t10253: F, t2771: F, t10388: F, t192: F, t852: F, t10478: F, t2: F, t10410: F, t1775: F, t2772: F) -> (F, F, F, F, F, F) {
    let t10604 = t10603 * t10249;
    let t10607 = t2771 * t10253;
    let t10611 = t192 * t852 * t10388;
    let t10613 = t10478 * t2;
    let t10614 = t10613 * t10410;
    let t10617 = t1775 * t2772;
    (t10604, t10607, t10611, t10613, t10614, t10617)
}
