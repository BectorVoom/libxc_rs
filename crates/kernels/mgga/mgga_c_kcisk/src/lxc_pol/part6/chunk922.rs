//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 922/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk922<F: Float>(t29367: F, t5289: F, t2587: F, t9062: F, t28303: F, t7303: F, t28749: F, t7430: F, t7429: F, t17969: F, t9047: F, t17933: F, t9072: F) -> (F, F, F, F, F, F) {
    let t29368 = t5289 * t29367;
    let t29370 = t9062 * t2587;
    let t29372 = t7303 * t28303;
    let t29373 = t5289 * t29372;
    let t29375 = t7430 * t28749;
    let t29376 = t7429 * t29375;
    let t29378 = t17969 * t9047;
    let t29380 = t17933 * t9072;
    (t29368, t29370, t29373, t29376, t29378, t29380)
}
