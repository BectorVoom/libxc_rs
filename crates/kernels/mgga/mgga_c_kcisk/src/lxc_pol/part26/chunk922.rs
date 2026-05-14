//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 922/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk922<F: Float>(t25446: F, t457: F, t1471: F, t25465: F, t25469: F, t158: F, t165: F, t173: F, t25455: F, t25458: F, t25461: F, t25466: F, t25470: F, t25473: F, t25476: F, t25479: F, t25482: F, t25485: F, t25487: F, t25489: F, t25491: F, t25493: F, t25495: F, t5816: F, t5823: F, t5827: F) -> (F,) {
    let t25497 = t457 * t25446;
    let t25500 = t1471 * t25465;
    let t25503 = t457 * t25469;
    let t25506 = 0.7026e-2 * t158 * t25455 + 0.1171e-2 * t158 * t25458 + 0.317e-2 * t165 * t25461 - 0.17611111111111111111e-3 * t165 * t25466 - 0.21133333333333333333e-2 * t5816 * t25470 - 0.10082625e-4 * t173 * t25473 - 0.672175e-5 * t173 * t25476 + 0.22405833333333333333e-5 * t173 * t25479 - 0.26887e-4 * t5823 * t25482 + 0.23526125e-4 * t25485 + 0.4684e-2 * t25487 - 0.117630625e-4 * t25489 + 0.15684083333333333333e-4 * t25491 - 0.9368e-2 * t25493 + 0.26416666666666666667e-2 * t25495 - 0.7026e-2 * t158 * t25497 + 0.78066666666666666667e-3 * t158 * t25500 + 0.4684e-2 * t5827 * t25503;
    (t25506,)
}
