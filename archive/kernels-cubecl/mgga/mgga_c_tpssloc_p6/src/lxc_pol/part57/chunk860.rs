//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 860/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk860<F: Float>(t1437: F, t8307: F, t7440: F, t8513: F, t191: F, t192: F, t7681: F, t3701: F, t7752: F, t4028: F, t8326: F, t7676: F) -> (F, F, F, F, F, F, F) {
    let t33106 = t8307 * t1437;
    let t33114 = t8307 * t7440;
    let t33115 = t8513 * t33114;
    let t33133 = t7681 * t191 * t192;
    let t33136 = t3701 * t7752;
    let t33151 = t4028 * t8326;
    let t33152 = F::cast_from(2.0_f64) * t33151;
    let t33153 = t7676 * t8326;
    (t33106, t33115, t33133, t33136, t33151, t33152, t33153)
}
