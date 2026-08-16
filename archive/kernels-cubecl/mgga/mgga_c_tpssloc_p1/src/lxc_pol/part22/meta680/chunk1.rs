//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2244/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2244<F: Float>(t3082: F, t5905: F, t10403: F, t10422: F, t18035: F, t17906: F, t3048: F, t1041: F, t248: F, t43338: F, t5677: F, t3070: F, t43198: F, t5908: F) -> (F, F, F, F, F) {
    let t62360 = t5905 * t3082;
    let t62418 = t10403 * t10422 * t18035;
    let t62441 = t3048 * t17906;
    let t62445 = t1041 * t248 * t43338 * t5677;
    let t62494 = t3070 * t43198 * t5908;
    (t62360, t62418, t62441, t62445, t62494)
}
