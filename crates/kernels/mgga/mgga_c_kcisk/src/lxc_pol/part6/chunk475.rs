//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 475/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk475<F: Float>(t381: F, t79: F, t3784: F, t492: F, t306: F, t476: F, t140: F, t430: F, t480: F, t11: F, t139: F) -> (F, F, F, F, F) {
    let t4231 = t79 * t381;
    let t4235 = t3784 * t492;
    let t4253 = t476 * t306;
    let t4264 = F::cast_from(0.88437037037037037037e-2_f64) * t140 * t430 * t480;
    let t4265 = t139 * t11;
    (t4231, t4235, t4253, t4264, t4265)
}
