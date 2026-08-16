//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1326/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1326<F: Float>(t1983: F, t2019: F, t83695: F, t2235: F, t2244: F, t71: F, t9338: F, t33: F, t39046: F, t608: F, t9239: F, t1864: F, t2241: F) -> (F, F, F, F, F, F) {
    let t83698 = F::cast_from(6.0_f64) * t1983 * t2019 * t83695;
    let t83699 = t2235 * t2244;
    let t83706 = t71 * t9338;
    let t83710 = t39046 * t33;
    let t83717 = t9239 * t608;
    let t83718 = t1864 * t2241;
    (t83698, t83699, t83706, t83710, t83717, t83718)
}
