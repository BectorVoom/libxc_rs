//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 735/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk735<F: Float>(t1423: F, t7784: F, t1964: F, t9419: F, t823: F, t2089: F, t40: F, t7291: F, t10007: F, t10012: F, t588: F, t7068: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22256 = t1423 * t7784;
    let t22537 = t1964 * t9419;
    let t22542 = t823 * t9419;
    let t22623 = t40 * t2089;
    let t22624 = t22623 * t7291;
    let t22629 = t10007 * t7291;
    let t22634 = t10012 * t7291;
    let t22665 = t588 * t2089;
    let t22980 = t10007 * t7068;
    (t22256, t22537, t22542, t22623, t22624, t22629, t22634, t22665, t22980)
}
